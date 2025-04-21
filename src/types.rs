use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Google Maps TSP Solver")]
pub struct Args {
    #[arg(long)]
    pub csv: String,

    #[arg(long)]
    pub api_key: String,

    #[arg(long)]
    pub start: String,

    #[arg(long)]
    pub end: String,

    #[arg(long, value_enum, default_value = "time")]
    pub mode: Mode,

    #[arg(long, value_enum, default_value = "held-karp")]
    pub algorithm: Algorithm,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Algorithm {
    HeldKarp,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Mode {
    Distance,
    Time,
}

#[derive(serde::Deserialize, Debug)]
pub struct CsvEntry {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "URL")]
    pub url: String,
}
