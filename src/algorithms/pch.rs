//! PCH (Position-Candidate-Hypothesis) path improver.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::core::{Point, calculate_cycle_distance};

type SparseMatrix = HashMap<(usize, usize, usize), u64>;

struct SharedPchMatrix {
    counts: SparseMatrix,
    total_permutations: u64,
    total_hypotheses: u64,
    best_improvements: u64,
    best_path: Vec<usize>,
    best_distance: f64,
    best_per_start: Vec<(Vec<usize>, f64)>,
}

fn build_candidate_matrix<P: Point>(points: &[P], top_candidates: usize) -> Vec<Vec<usize>> {
    let n = points.len();
    let mut candidate_matrix = vec![vec![0usize; top_candidates]; n];

    for current_pos in 0..n {
        let mut dists = Vec::with_capacity(n);
        for candidate in 0..n {
            if candidate != current_pos {
                let dx = points[current_pos].x() - points[candidate].x();
                let dy = points[current_pos].y() - points[candidate].y();
                let dist = (dx * dx + dy * dy).sqrt();
                dists.push((candidate, dist));
            }
        }
        dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        for i in 0..top_candidates.min(dists.len()) {
            candidate_matrix[current_pos][i] = dists[i].0;
        }
    }
    candidate_matrix
}

/// Improves ANY TSP path using the PCH (Position-Candidate-Hypothesis) algorithm.
///
/// PCH is a universal path improver that works with ANY existing solution.
/// It analyzes transition frequencies and synthesizes a statistically better path.
///
/// # Arguments
/// * `points` - Slice of points implementing the `Point` trait
/// * `initial_path` - ANY existing TSP path (from ANY algorithm)
/// * `initial_distance` - Distance of the initial path
/// * `permutations_per_hypothesis` - Number of random swaps per hypothesis (default: 50)
/// * `batch_size` - Batch size for thread synchronization (default: 1000)
/// * `time_limit` - Time limit in seconds (default: 30)
/// * `top_candidates` - Number of top candidates per position (default: 300)
/// * `num_agents` - Number of parallel agents (default: 12)
///
/// # Returns
/// * `(f64, Vec<usize>)` - (Improved distance, Improved path)
///
/// # Example
/// ```
/// use smart_pch_tsp::{City, pch_improve};
///
/// let points = vec![
///     City { x: 0.0, y: 0.0 },
///     City { x: 1.0, y: 0.0 },
///     City { x: 0.0, y: 1.0 },
/// ];
///
/// let initial_path = vec![0, 1, 2, 0];
/// let initial_distance = 4.0;
///
/// let (improved_distance, improved_path) = pch_improve(
///     &points,
///     &initial_path,
///     initial_distance,
///     50, 1000, 30, 300, 12
/// );
///
/// assert!(improved_distance <= initial_distance);
/// ```
pub fn pch_improve<P: Point>(
    points: &[P],
    initial_path: &[usize],
    initial_distance: f64,
    permutations_per_hypothesis: usize,
    batch_size: u64,
    time_limit: u64,
    top_candidates: usize,
    num_agents: usize,
) -> (f64, Vec<usize>) {
    let n = points.len();
    if n == 0 {
        return (0.0, vec![]);
    }

    if n == 1 {
        return (0.0, vec![0, 0]);
    }

    let points_arc = Arc::new(points.to_vec());
    let candidate_matrix = Arc::new(build_candidate_matrix(points, top_candidates));

    let base_path = initial_path.to_vec();
    let base_distance = initial_distance;

    let global_matrix = Arc::new(Mutex::new(SharedPchMatrix {
        counts: HashMap::new(),
        total_permutations: 0,
        total_hypotheses: 0,
        best_improvements: 0,
        best_path: base_path.clone(),
        best_distance: base_distance,
        best_per_start: vec![(Vec::new(), f64::MAX); n],
    }));
    let pch_running = Arc::new(AtomicBool::new(true));
    let mut pch_handles = vec![];

    for agent_id in 0..num_agents {
        let points_ref = Arc::clone(&points_arc);
        let matrix_ref = Arc::clone(&global_matrix);
        let cand_ref = Arc::clone(&candidate_matrix);
        let running_ref = Arc::clone(&pch_running);
        let base_path_ref = base_path.clone();

        let handle = thread::spawn(move || {
            let mut seed: u32 = (agent_id + 1) as u32 * 2468 + agent_id as u32 * 12345;
            let mut next_random = move || {
                seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
                seed
            };

            let mut local_counts: SparseMatrix = HashMap::new();
            let mut local_perm_count = 0u64;
            let mut local_hypotheses = 0u64;
            let mut local_improvements = 0u64;

            let mut local_best_per_start: Vec<(Vec<usize>, f64)> = vec![(Vec::new(), f64::MAX); n];
            let mut local_global_best_path = base_path_ref.clone();
            let mut local_global_best_distance =
                calculate_cycle_distance(&local_global_best_path, &points_ref);

            while running_ref.load(Ordering::Relaxed) {
                for start_city in 0..n {
                    local_hypotheses += 1;

                    let mut closed_path = Vec::with_capacity(n + 1);
                    let mut visited = vec![false; n];
                    closed_path.push(start_city);
                    visited[start_city] = true;

                    for _pos in 1..n {
                        let current_city = *closed_path.last().unwrap();
                        let mut chosen = cand_ref[current_city][0];

                        let mut found = false;
                        for i in 0..top_candidates {
                            let candidate = cand_ref[current_city][i];
                            if !visited[candidate] {
                                chosen = candidate;
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            let mut min_dist = f64::MAX;
                            for candidate in 0..n {
                                if !visited[candidate] {
                                    let dx =
                                        points_ref[current_city].x() - points_ref[candidate].x();
                                    let dy =
                                        points_ref[current_city].y() - points_ref[candidate].y();
                                    let dist = (dx * dx + dy * dy).sqrt();
                                    if dist < min_dist {
                                        min_dist = dist;
                                        chosen = candidate;
                                    }
                                }
                            }
                        }

                        visited[chosen] = true;
                        closed_path.push(chosen);
                    }

                    closed_path.push(closed_path[0]);

                    let base_dist = calculate_cycle_distance(&closed_path, &points_ref);

                    if base_dist < local_best_per_start[start_city].1 {
                        local_best_per_start[start_city] = (closed_path.clone(), base_dist);
                        local_improvements += 1;
                    }

                    if base_dist < local_global_best_distance {
                        local_global_best_distance = base_dist;
                        local_global_best_path = closed_path.clone();
                        local_improvements += 1;
                    }

                    for _variant in 0..permutations_per_hypothesis {
                        let mut hyp_path = local_global_best_path.clone();

                        let pos1 = 1 + (next_random() as usize) % (n - 1);
                        let pos2 = 1 + (next_random() as usize) % (n - 1);

                        if pos1 != pos2 && pos1 < n && pos2 < n {
                            let city1 = hyp_path[pos1];
                            let city2 = hyp_path[pos2];
                            let prev_city1 = hyp_path[pos1 - 1];
                            let prev_city2 = hyp_path[pos2 - 1];

                            let mut is_candidate1 = false;
                            for i in 0..top_candidates {
                                if cand_ref[prev_city1][i] == city1 {
                                    is_candidate1 = true;
                                    break;
                                }
                            }

                            let mut is_candidate2 = false;
                            for i in 0..top_candidates {
                                if cand_ref[prev_city2][i] == city2 {
                                    is_candidate2 = true;
                                    break;
                                }
                            }

                            if is_candidate1 && is_candidate2 {
                                hyp_path.swap(pos1, pos2);
                                let hyp_dist = calculate_cycle_distance(&hyp_path, &points_ref);

                                let is_improvement = hyp_dist < local_global_best_distance;
                                let weight = if is_improvement { 100 } else { 1 };

                                if is_improvement {
                                    local_global_best_distance = hyp_dist;
                                    local_global_best_path = hyp_path.clone();
                                    local_improvements += 1;

                                    if hyp_dist < local_best_per_start[start_city].1 {
                                        local_best_per_start[start_city] =
                                            (hyp_path.clone(), hyp_dist);
                                    }
                                }

                                for pos in 1..n {
                                    let c1 = hyp_path[pos - 1];
                                    let c2 = hyp_path[pos];
                                    let key = (pos, c1, c2);
                                    *local_counts.entry(key).or_insert(0) += weight;
                                }

                                local_perm_count += 1;
                            }
                        }
                    }
                }

                if local_perm_count >= batch_size {
                    let mut global = matrix_ref.lock().unwrap();
                    global.total_permutations += local_perm_count;
                    global.total_hypotheses += local_hypotheses;
                    global.best_improvements += local_improvements;

                    if local_global_best_distance < global.best_distance {
                        global.best_distance = local_global_best_distance;
                        global.best_path = local_global_best_path.clone();
                    }

                    for i in 0..n {
                        if local_best_per_start[i].1 < global.best_per_start[i].1 {
                            global.best_per_start[i] = local_best_per_start[i].clone();
                        }
                    }

                    for ((pos, c1, c2), weight) in local_counts.drain() {
                        *global.counts.entry((pos, c1, c2)).or_insert(0) += weight;
                    }
                    local_perm_count = 0;
                    local_hypotheses = 0;
                    local_improvements = 0;
                }
            }

            let mut global = matrix_ref.lock().unwrap();
            global.total_permutations += local_perm_count;
            global.total_hypotheses += local_hypotheses;
            global.best_improvements += local_improvements;

            if local_global_best_distance < global.best_distance {
                global.best_distance = local_global_best_distance;
                global.best_path = local_global_best_path.clone();
            }

            for i in 0..n {
                if local_best_per_start[i].1 < global.best_per_start[i].1 {
                    global.best_per_start[i] = local_best_per_start[i].clone();
                }
            }

            for ((pos, c1, c2), weight) in local_counts {
                *global.counts.entry((pos, c1, c2)).or_insert(0) += weight;
            }
        });
        pch_handles.push(handle);
    }

    let pch_start = Instant::now();
    let mut pch_last_print = Instant::now();
    let mut prev_perms = 0u64;

    while pch_start.elapsed() < Duration::from_secs(time_limit) {
        thread::sleep(Duration::from_millis(500));
        if pch_last_print.elapsed() >= Duration::from_secs(1) {
            let matrix = global_matrix.lock().unwrap();
            let rate = matrix.total_permutations - prev_perms;
            prev_perms = matrix.total_permutations;
            println!(
                "   PCH: {:.1}s | Hypotheses: {} | Permutations: {} | Improvements: {} | Records: {} | Best: {:.2} | Speed: {}/s",
                pch_start.elapsed().as_secs_f64(),
                matrix.total_hypotheses,
                matrix.total_permutations,
                matrix.best_improvements,
                matrix.counts.len(),
                matrix.best_distance,
                rate
            );
            pch_last_print = Instant::now();
        }
    }

    pch_running.store(false, Ordering::Relaxed);
    for handle in pch_handles {
        handle.join().unwrap();
    }

    let final_matrix = global_matrix.lock().unwrap();

    let mut synthesized_path = Vec::with_capacity(n + 1);
    let mut position_assigned = vec![false; n];

    let mut best_start_city = 0;
    let mut max_start_weight = 0;
    for c1 in 0..n {
        for c2 in 0..n {
            let key = (1, c1, c2);
            if let Some(&weight) = final_matrix.counts.get(&key) {
                if weight > max_start_weight {
                    max_start_weight = weight;
                    best_start_city = c1;
                }
            }
        }
    }

    synthesized_path.push(best_start_city);
    position_assigned[best_start_city] = true;

    for pos in 1..n {
        let current_city = *synthesized_path.last().unwrap();
        let mut best_next = 0;
        let mut max_weight = 0;

        for next_candidate in 0..n {
            if !position_assigned[next_candidate] {
                let key = (pos, current_city, next_candidate);
                if let Some(&weight) = final_matrix.counts.get(&key) {
                    if weight > max_weight {
                        max_weight = weight;
                        best_next = next_candidate;
                    }
                }
            }
        }

        if max_weight == 0 {
            let mut min_d = f64::MAX;
            for fallback in 0..n {
                if !position_assigned[fallback] {
                    let dx = points[current_city].x() - points[fallback].x();
                    let dy = points[current_city].y() - points[fallback].y();
                    let d = (dx * dx + dy * dy).sqrt();
                    if d < min_d {
                        min_d = d;
                        best_next = fallback;
                    }
                }
            }
        }

        synthesized_path.push(best_next);
        position_assigned[best_next] = true;
    }

    synthesized_path.push(synthesized_path[0]);

    let synthesized_distance = calculate_cycle_distance(&synthesized_path, points);
    let best_found_path = final_matrix.best_path.clone();
    let best_found_distance = final_matrix.best_distance;

    let final_path = if synthesized_distance < best_found_distance {
        synthesized_path
    } else {
        best_found_path
    };

    let final_distance = if synthesized_distance < best_found_distance {
        synthesized_distance
    } else {
        best_found_distance
    };

    (final_distance, final_path)
}
