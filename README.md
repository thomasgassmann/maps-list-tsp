# Google Maps TSP Solver

## How to use

You will need a Google API key with the following APIs enabled:

- Places API
- Geocoding API
- Distance Matrix API

Then use the API key to run the TSP on the csv file:

```bash
cargo run --release -- --csv maps/example.csv --api-key <your-api-key> --start "Panda Noodle Bar" --end "Arches National Park Visitor Center"
```

You can use `--mode distance` to calculate the TSP on distance values. The default (`--mode time`) minimizes time.

You can also specify the algorithm to be used via the `--algorithm` flag. The default is `held-karp`. You have the following options:

- Held Karp (`held-karp`): time: $O(n^2 2^n)$, space: $O(2^n)$
- Brute Force (`brute-force`): time $O(n!)$, space: $O(n^2)$, parallelized

Note that the number of API requests scales quadratically with the number of nodes in the csv!

## Getting a csv file or your list

To get the CSV file of your saved places, you will need to use Google Takeout (select "Saved").

## Example output

```text
Running algorithm: held-karp

Optimal Path:
-------------
1. Panda Noodle Bar
2. Silverton
3. Park Avenue Trailhead
4. La Sal Mountains Viewpoint
5. Courthouse Towers Viewpoint and Trailhead
6. Balanced Rock Trailhead
7. Upper Delicate Arch Viewpoint Trail
8. Devil's Garden Trail
9. Fiery Furnace Viewpoint
10. Salt Valley Overlook
11. Panorama Point
12. Double Arch Viewpoint and Trail
13. Arches National Park Visitor Center

Segment Distances:
------------------
Panda Noodle Bar -> Silverton: 1148 minutes
Silverton -> Park Avenue Trailhead: 231 minutes
Park Avenue Trailhead -> La Sal Mountains Viewpoint: 2 minutes
La Sal Mountains Viewpoint -> Courthouse Towers Viewpoint and Trailhead: 3 minutes
Courthouse Towers Viewpoint and Trailhead -> Balanced Rock Trailhead: 9 minutes
Balanced Rock Trailhead -> Upper Delicate Arch Viewpoint Trail: 9 minutes
Upper Delicate Arch Viewpoint Trail -> Devil's Garden Trail: 15 minutes
Devil's Garden Trail -> Fiery Furnace Viewpoint: 8 minutes
Fiery Furnace Viewpoint -> Salt Valley Overlook: 3 minutes
Salt Valley Overlook -> Panorama Point: 7 minutes
Panorama Point -> Double Arch Viewpoint and Trail: 6 minutes
Double Arch Viewpoint and Trail -> Arches National Park Visitor Center: 19 minutes

Optimal distance from "Panda Noodle Bar" to "Arches National Park Visitor Center": 1460 minutes
```
