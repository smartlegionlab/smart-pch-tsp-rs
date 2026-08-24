use smart_pch_tsp::{City, pch_improve};
use std::time::Instant;

fn generate_cities(count: usize) -> Vec<City> {
    let mut seed = 42u64;
    let mut cities = Vec::with_capacity(count);

    for _ in 0..count {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let x = (seed % 1000) as f64;
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        let y = (seed % 1000) as f64;
        cities.push(City { x, y });
    }

    cities
}

fn greedy_path(cities: &[City]) -> (f64, Vec<usize>) {
    let n = cities.len();
    let mut best_path = Vec::with_capacity(n + 1);
    let mut best_distance = f64::MAX;

    for start in 0..n.min(10) {
        let mut path = Vec::with_capacity(n + 1);
        let mut visited = vec![false; n];
        let mut current = start;
        path.push(current);
        visited[current] = true;

        for _ in 0..n - 1 {
            let mut next_city = 0;
            let mut min_dist = f64::MAX;

            for candidate in 0..n {
                if !visited[candidate] {
                    let dx = cities[current].x - cities[candidate].x;
                    let dy = cities[current].y - cities[candidate].y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < min_dist {
                        min_dist = dist;
                        next_city = candidate;
                    }
                }
            }

            visited[next_city] = true;
            path.push(next_city);
            current = next_city;
        }

        path.push(path[0]);

        let mut dist = 0.0;
        for i in 0..path.len() - 1 {
            let dx = cities[path[i]].x - cities[path[i + 1]].x;
            let dy = cities[path[i]].y - cities[path[i + 1]].y;
            dist += (dx * dx + dy * dy).sqrt();
        }

        if dist < best_distance {
            best_distance = dist;
            best_path = path;
        }
    }

    (best_distance, best_path)
}

fn print_path_preview(path: &[usize], name: &str) {
    let total = path.len();
    let first_n = 5;
    let last_n = 5;

    print!("   Path {}: ", name);

    if total <= 10 {
        for i in 0..total {
            print!("{}", path[i]);
            if i < total - 1 {
                print!(" -> ");
            }
        }
        println!(" ({} total, closed)", total);
        return;
    }

    for i in 0..first_n {
        print!("{}", path[i]);
        if i < first_n - 1 {
            print!(" -> ");
        }
    }

    print!(" -> ... -> ");

    for i in (total - last_n)..total {
        print!("{}", path[i]);
        if i < total - 1 {
            print!(" -> ");
        }
    }

    println!(" ({} total, closed)", total);
}

fn main() {
    println!("================================================================================");
    println!("PCH PATH IMPROVER - Improving Greedy Solution");
    println!("================================================================================");

    let num_cities = 1000;
    println!("\nGenerating {} cities...", num_cities);
    let cities = generate_cities(num_cities);

    println!("\n[1/2] GREEDY (Initial Solution)");
    let start = Instant::now();
    let (initial_distance, initial_path) = greedy_path(&cities);
    let initial_time = start.elapsed().as_secs_f64();
    println!("   Distance: {:.2}", initial_distance);
    println!("   Time: {:.3}s", initial_time);
    print_path_preview(&initial_path, "GREEDY");

    println!("\n[2/2] PCH IMPROVER");
    println!("   Parameters:");
    println!("     Permutations:        50");
    println!("     Batch size:          1000");
    println!("     Time limit:          5s (quick demo)");
    println!("     Top candidates:      300");
    println!("     Agents:              12");

    let start = Instant::now();
    let (improved_distance, improved_path) = pch_improve(
        &cities,
        &initial_path,
        initial_distance,
        50,
        1000,
        5,
        300,
        12,
    );
    let improve_time = start.elapsed().as_secs_f64();

    println!("\n   Final Distance: {:.2}", improved_distance);
    println!("   Time: {:.3}s", improve_time);
    print_path_preview(&improved_path, "PCH IMPROVED");

    println!("\n================================================================================");
    println!("IMPROVEMENT SUMMARY");
    println!("================================================================================");
    println!("   Initial Distance:   {:.2}", initial_distance);
    println!("   Improved Distance:  {:.2}", improved_distance);

    let diff = initial_distance - improved_distance;
    let pct = (diff / initial_distance) * 100.0;

    if diff > 0.0 {
        println!("\n   Improvement: {:.2} ({:.1}%)", diff, pct);
        println!("   Time: {:.3}s vs {:.3}s", initial_time, improve_time);
        println!("\n   ✅ PCH SUCCESSFULLY IMPROVED THE PATH!");
    } else {
        println!("\n   ⚠️ No improvement found (try increasing time_limit)");
    }
    println!("================================================================================");
}
