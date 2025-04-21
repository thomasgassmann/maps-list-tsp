use google_maps::prelude::*;
use log::{debug, warn};
use regex::Regex;
use serde_json::Value;

use crate::types::{CsvEntry, Mode};

pub async fn get_waypoints(
    entries: &Vec<CsvEntry>,
    api_key: &str,
) -> Result<Vec<Waypoint>, Box<dyn std::error::Error>> {
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

pub async fn get_distance_matrix(
    waypoints: &Vec<Waypoint>,
    api_key: &str,
    mode: &Mode,
) -> Result<Vec<Vec<i64>>, Box<dyn std::error::Error>> {
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

        for j in (i..n).step_by(CHUNK_SIZE) {
            let j_end = (j + CHUNK_SIZE).min(n);
            let destinations = &waypoints[j..j_end];

            debug!(
                "Calculating distance matrix: orig: {:?}, dest: {:?}",
                origins, destinations
            );
            let dm = maps_client
                .distance_matrix(origins, destinations)
                .execute()
                .await?;

            for (chunk_i, row) in dm.rows.into_iter().enumerate() {
                let current_i = i + chunk_i;
                for (chunk_j, elem) in row.elements.into_iter().enumerate() {
                    let current_j = j + chunk_j;

                    match mode {
                        Mode::Distance => {
                            if let Some(distance) = elem.distance {
                                let distance_meters = distance.value as i64;
                                dist[current_i][current_j] = distance_meters;
                                dist[current_j][current_i] = distance_meters;
                            } else {
                                warn!("No distance available from {} to {}", current_i, current_j);
                            }
                        }
                        Mode::Time => {
                            if let Some(duration) = elem.duration {
                                let duration_minutes = duration.value.num_minutes();
                                dist[current_i][current_j] = duration_minutes;
                                dist[current_j][current_i] = duration_minutes;
                            } else {
                                warn!("No duration available from {} to {}", current_i, current_j);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(dist)
}
