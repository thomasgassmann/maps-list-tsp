use clap::Parser;
use csv::Reader;
use google_maps::prelude::*;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use log::{warn, debug};

#[derive(Parser, Debug)]
#[command(version, about = "Google Maps TSP Solver")]
struct Args {
    #[arg(long)]
    csv: String,

    #[arg(long)]
    api_key: String,

    #[arg(long)]
    start: String,

    #[arg(long)]
    end: String,
}

#[derive(serde::Deserialize, Debug)]
struct CsvEntry {
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "URL")]
    url: String,
}

async fn get_waypoints(entries: &Vec<CsvEntry>, api_key: &str) -> Result<Vec<Waypoint>, Box<dyn std::error::Error>> {
    let re_pid = Regex::new(r"!1s([^!]+)").unwrap();
    let re_search = Regex::new(r"/search/([-0-9.]+),([-0-9.]+)").unwrap();
    let re_ftid = Regex::new(r"^0x[0-9A-Fa-f]+:0x[0-9A-Fa-f]+$").unwrap();
    
    let maps_client = GoogleMapsClient::try_new(api_key)?;
    let mut waypoints = Vec::with_capacity(entries.len());
    for e in entries {
        if let Some(cap) = re_pid.captures(&e.url) {
            let token = &cap[1];
            if !re_ftid.is_match(token) {
                return Err(format!("Invalid ftid found: {}", token).into());
            }

            // take the second ftid
            let cid = token.split(':').skip(1).next().unwrap();

            let details_url = format!(
                "https://maps.googleapis.com/maps/api/place/details/json?cid={}&key={}",
                cid, api_key
            );

            let response = maps_client
                .reqwest_client
                .get(&details_url)
                .send()
                .await?
                .json::<Value>()
                .await?;

            let result = response.get("result").expect("result not found");
            let geometry = result.get("geometry").expect("geometry not found");
            let location = geometry.get("location").expect("location not found");
            let lat = location
                .get("lat")
                .and_then(|v| v.as_f64())
                .ok_or("lat not found")? as f32;
            let lng = location
                .get("lng")
                .and_then(|v| v.as_f64())
                .ok_or("lng not found")? as f32;
            waypoints.push(Waypoint::try_from_f32(lat, lng)?);
        } else if let Some(cap) = re_search.captures(&e.url) {
            let lat: f32 = cap[1].parse().unwrap();
            let lng: f32 = cap[2].parse().unwrap();
            waypoints.push(Waypoint::try_from_f32(lat, lng)?);
        } else {
            return Err(format!("Unrecognized URL: {}", e.url).into());
        }
    }

    Ok(waypoints)
}

async fn get_distance_matrix(waypoints: &Vec<Waypoint>, api_key: &str) -> Result<Vec<Vec<i64>>, Box<dyn std::error::Error>> {
    let n = waypoints.len();
    let mut dist = vec![vec![i64::MAX; n]; n];
    let maps_client = GoogleMapsClient::try_new(api_key)?;
    
    for i in 0..n {
        dist[i][i] = 0;
    }
    
    const CHUNK_SIZE: usize = 10;
    for i in (0..n).step_by(CHUNK_SIZE) {
        let i_end = (i + CHUNK_SIZE).min(n);
        let origins = &waypoints[i..i_end];
        
        for j in (0..n).step_by(CHUNK_SIZE) {
            let j_end = (j + CHUNK_SIZE).min(n);
            let destinations = &waypoints[j..j_end];

            debug!("Calculating distance matrix: orig: {:?}, dest: {:?}", origins, destinations);
            let dm = maps_client
                .distance_matrix(origins, destinations)
                .execute()
                .await?;
            
            for (chunk_i, row) in dm.rows.into_iter().enumerate() {
                let global_i = i + chunk_i;
                for (chunk_j, elem) in row.elements.into_iter().enumerate() {
                    let global_j = j + chunk_j;

                    if let Some(duration) = elem.duration {
                        dist[global_i][global_j] = duration.value.num_minutes();
                    } else {
                        warn!("No distance available from {} to {}", global_i, global_j);
                    }
                }
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    }

    Ok(dist)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    let mut csv_reader = Reader::from_path(&args.csv)?;
    let entries: Vec<CsvEntry> = csv_reader.deserialize().collect::<Result<_, _>>()?;
    
    let title_to_idx: HashMap<_, _> = entries
        .iter()
        .enumerate()
        .map(|(i, e)| (&e.title, i))
        .collect();
    let idx_to_title: HashMap<_, _> = title_to_idx
        .iter()
        .map(|(title, &idx)| (idx, *title))
        .collect();
    
    let &start = title_to_idx.get(&args.start).expect("start not found");
    let &end = title_to_idx.get(&args.end).expect("end not found");

    let waypoints = get_waypoints(&entries, &args.api_key).await?;

    let dist = get_distance_matrix(&waypoints, &args.api_key).await?;

    let n = waypoints.len();
    let full_mask = (1 << n) - 1;
    let mut dp = vec![vec![i64::MAX; 1 << n]; n];
    let mut parent = vec![vec![usize::MAX; 1 << n]; n];
    
    dp[start][1 << start] = 0;
    for mask in 0..=full_mask {
        for u in 0..n {
            let c = dp[u][mask];
            if c == i64::MAX {
                continue;
            }

            for v in 0..n {
                if mask & (1 << v) == 0 {
                    let nxt = mask | (1 << v);
                    
                    if dist[u][v] == i64::MAX {
                        continue;
                    }
                    
                    if let Some(new_cost) = c.checked_add(dist[u][v]) {
                        if new_cost < dp[v][nxt] {
                            dp[v][nxt] = new_cost;
                            parent[v][nxt] = u;
                        }
                    }
                }
            }
        }
    }
    
    let best = dp[end][full_mask];
    if best == i64::MAX {
        return Err(format!("No valid path found from \"{}\" to \"{}\"!", args.start, args.end).into());
    }
    
    println!(
        "Optimal distance from \"{}\" to \"{}\": {} minutes",
        args.start, args.end, best
    );

    let mut path = Vec::new();
    let mut current = end;
    let mut mask = full_mask;
    
    path.push(current);
    
    while mask != (1 << start) {
        let prev = parent[current][mask];
        if prev == usize::MAX {
            warn!("Warning: Cannot fully reconstruct path!");
            break;
        }
        
        path.push(prev);
        mask &= !(1 << current);
        current = prev;
    }
    
    path.reverse();
    
    println!("\nOptimal Path:");
    println!("-------------");
    
    for (i, &node) in path.iter().enumerate() {
        let title = idx_to_title.get(&node).expect("index not found in mapping");
        println!("{}. {}", i + 1, title);
    }
    
    println!("\nSegment Distances:");
    println!("-----------------");
    
    for i in 0..path.len() - 1 {
        let from = path[i];
        let to = path[i + 1];
        let segment_dist = dist[from][to];
        
        let from_title = idx_to_title.get(&from).expect("index not found in mapping");
        let to_title = idx_to_title.get(&to).expect("index not found in mapping");
        
        println!("{} -> {}: {} minutes", from_title, to_title, segment_dist);
    }

    Ok(())
}