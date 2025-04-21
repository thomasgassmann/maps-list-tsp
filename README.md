# Google Maps TSP Solver

## How to use

You will need a Google API key with the following APIs enabled:

- Places API
- Geocoding API
- Distance Matrix API

Then use the API key to run the TSP on $O(n^2 2^n)$ on the csv file:

```bash
cargo run -- --csv maps/example.csv --api-key <your-api-key> --start "Panda Noodle Bar" --end "Arches National Park Visitor Center"
```

You can use `--mode distance` to calculate the TSP on distance values. The default (`--mode time`) minimizes time.

## Getting a csv file or your list

To get the CSV file of your saved places, you will need to use Google Takeout (select "Saved").

## Example output

```text
Optimal distance from "Panda Noodle Bar" to "Arches National Park Visitor Center": 3675 minutes

Optimal Path:
-------------
1. Panda Noodle Bar
2. University of Washington
3. Dough Zone Dumpling House University District
4. Columbia River Gorge
5. Palouse Falls
6. 46°26'58.9"N 9°48'21.7"E
7. Sinks Canyon State Park
8. Rocky Mountain National Park
9. Goblin Valley State Park
10. Kodachrome Basin State Park
11. Cathedral Gorge State Park
12. The Wave
13. Mesa Verde National Park
14. Canyonlands National Park
15. Upper Delicate Arch Viewpoint Trail
16. Devil's Garden Trail
17. Fiery Furnace Viewpoint
18. Salt Valley Overlook
19. Panorama Point
20. Double Arch Viewpoint and Trail
21. Balanced Rock Trailhead
22. Courthouse Towers Viewpoint and Trailhead
23. La Sal Mountains Viewpoint
24. Park Avenue Trailhead
25. Arches National Park Visitor Center

Segment Distances:
------------------
Panda Noodle Bar -> University of Washington: 5 minutes
University of Washington -> Dough Zone Dumpling House University District: 4 minutes
Dough Zone Dumpling House University District -> Columbia River Gorge: 209 minutes
Columbia River Gorge -> Palouse Falls: 228 minutes
Palouse Falls -> 46°26'58.9"N 9°48'21.7"E: 539 minutes
46°26'58.9"N 9°48'21.7"E -> Sinks Canyon State Park: 503 minutes
Sinks Canyon State Park -> Rocky Mountain National Park: 348 minutes
Rocky Mountain National Park -> Goblin Valley State Park: 440 minutes
Goblin Valley State Park -> Kodachrome Basin State Park: 221 minutes
Kodachrome Basin State Park -> Cathedral Gorge State Park: 193 minutes
Cathedral Gorge State Park -> The Wave: 240 minutes
The Wave -> Mesa Verde National Park: 307 minutes
Mesa Verde National Park -> Canyonlands National Park: 214 minutes
Canyonlands National Park -> Upper Delicate Arch Viewpoint Trail: 159 minutes
Upper Delicate Arch Viewpoint Trail -> Devil's Garden Trail: 15 minutes
Devil's Garden Trail -> Fiery Furnace Viewpoint: 8 minutes
Fiery Furnace Viewpoint -> Salt Valley Overlook: 3 minutes
Salt Valley Overlook -> Panorama Point: 7 minutes
Panorama Point -> Double Arch Viewpoint and Trail: 6 minutes
Double Arch Viewpoint and Trail -> Balanced Rock Trailhead: 7 minutes
Balanced Rock Trailhead -> Courthouse Towers Viewpoint and Trailhead: 9 minutes
Courthouse Towers Viewpoint and Trailhead -> La Sal Mountains Viewpoint: 3 minutes
La Sal Mountains Viewpoint -> Park Avenue Trailhead: 2 minutes
Park Avenue Trailhead -> Arches National Park Visitor Center: 5 minutes
```
