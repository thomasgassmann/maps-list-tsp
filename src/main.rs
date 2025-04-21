mod parse;
mod tsp;
mod types;

use clap::Parser;
use csv::Reader;
use parse::{get_distance_matrix, get_waypoints};
use std::collections::HashMap;
use types::{Args, CsvEntry, Mode};

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

    let dist = get_distance_matrix(&waypoints, &args.api_key, &args.mode).await?;
    for i in 0..waypoints.len() {
        if dist[i].iter().all(|&d| d == i64::MAX || d == 0) {
            return Err(format!(
                "Waypoint unreachable from any other point: {}",
                idx_to_title.get(&i).unwrap()
            )
            .into());
        }
    }

    let path = tsp::held_karp(&waypoints, start, end, &dist)?;
    let unit_name: &str = match args.mode {
        Mode::Distance => "meters",
        Mode::Time => "minutes",
    };

    println!("Optimal Path:");
    println!("-------------");

    for (i, &node) in path.iter().enumerate() {
        let title = idx_to_title.get(&node).expect("index not found in mapping");
        println!("{}. {}", i + 1, title);
    }

    println!("\nSegment Distances:");
    println!("------------------");

    let mut total_dist: i64 = 0;
    for i in 0..path.len() - 1 {
        let from = path[i];
        let to = path[i + 1];
        let segment_dist = dist[from][to];

        let from_title = idx_to_title.get(&from).expect("index not found in mapping");
        let to_title = idx_to_title.get(&to).expect("index not found in mapping");

        println!(
            "{} -> {}: {} {}",
            from_title, to_title, segment_dist, unit_name
        );

        total_dist += segment_dist;
    }

    println!(
        "\nOptimal distance from \"{}\" to \"{}\": {} {}",
        idx_to_title.get(&start).unwrap(),
        idx_to_title.get(&end).unwrap(),
        total_dist,
        unit_name
    );

    Ok(())
}
