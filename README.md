# Google Maps TSP Solver

## How to use

You will need a Google API key with the following APIs enabled:

- Places API
- Geocoding API
- Distance Matrix API

Then use the API key to run the TSP on the csv file:

```bash
cargo run -- --csv maps/example.csv --api-key <your-api-key> --start "Panda Noodle Bar" --end "Arches National Park Visitor Center" 
```

## Getting a csv file or your list

To get the CSV file of your saved places, you will need to use Google Takeout (selected "Saved").
