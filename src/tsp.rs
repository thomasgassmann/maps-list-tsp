use itertools::Itertools;
use rayon::prelude::*;

pub fn brute_force(
    n: usize,
    start: usize,
    end: usize,
    dist: &Vec<Vec<i64>>,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let intermediate_nodes: Vec<usize> = (0..n).filter(|&i| i != start).collect();
    let best = intermediate_nodes
        .clone()
        .par_iter()
        .map(|&first_intermediate_node| {
            let mut thread_min_dist: i64 = i64::MAX;
            let mut thread_best_path: Option<Vec<usize>> = None;

            let remaining_nodes: Vec<usize> = intermediate_nodes
                .iter()
                .copied()
                .filter(|&node| node != first_intermediate_node)
                .collect();

            for permutation in remaining_nodes.iter().permutations(remaining_nodes.len()) {
                let mut current_path: Vec<usize> = vec![start, first_intermediate_node];
                current_path.extend(permutation.into_iter().copied());
                current_path.push(end);

                let mut current_dist: i64 = 0;
                let mut possible_path = true;

                for i in 0..current_path.len() - 1 {
                    let u = current_path[i];
                    let v = current_path[i + 1];
                    let d = dist[u][v];
                    if current_dist > i64::MAX - d || d == i64::MAX {
                        current_dist = i64::MAX;
                        possible_path = false;
                        break;
                    }

                    current_dist += d;
                }

                if possible_path && current_dist < thread_min_dist {
                    thread_min_dist = current_dist;
                    thread_best_path = Some(current_path);
                }
            }

            (thread_min_dist, thread_best_path)
        })
        .min_by_key(|(dist, _)| *dist);

    match best {
        Some((min_dist, Some(path))) if min_dist != i64::MAX => Ok(path),
        _ => Err("No valid path found visiting all nodes with finite distance".into()),
    }
}

pub fn held_karp(
    n: usize,
    start: usize,
    end: usize,
    dist: &Vec<Vec<i64>>,
) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
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
        return Err(format!("No valid path found from \"{}\" to \"{}\"!", start, end).into());
    }

    let mut current = end;
    let mut mask = full_mask;

    let mut path = Vec::new();
    path.push(current);
    while mask != (1 << start) {
        let prev = parent[current][mask];
        path.insert(0, prev);
        mask &= !(1 << current);
        current = prev;
    }

    Ok(path)
}
