use smart_dynamic_gravity_tsp::dynamic_gravity_solve;
use smart_pch_tsp::{Point, pch_improve};
use std::time::Instant;

// New type wrapper for DG's City
#[derive(Clone, Copy, Debug)]
struct DgCity(smart_dynamic_gravity_tsp::City);

impl Point for DgCity {
    fn x(&self) -> f64 {
        self.0.x
    }
    fn y(&self) -> f64 {
        self.0.y
    }
}

fn generate_cities(seed_phrase: &str, count: usize) -> Vec<smart_dynamic_gravity_tsp::City> {
    let mut map_seed = 2166136261u64;
    for byte in seed_phrase.bytes() {
        map_seed = (map_seed ^ byte as u64).wrapping_mul(1099511628211);
    }
    let mut map_rand = move || {
        map_seed =
            (map_seed.wrapping_mul(6364136223846793005).wrapping_add(1)) & 0x7fffffffffffffff;
        map_seed
    };

    let mut cities = Vec::with_capacity(count);

    for _ in 0..count {
        let rx = (map_rand() % 500) as f64;
        let ry = (map_rand() % 500) as f64;
        cities.push(smart_dynamic_gravity_tsp::City { x: rx, y: ry });
    }

    cities
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
    println!("PCH PATH IMPROVER - Improving Dynamic Gravity Solution");
    println!("================================================================================");

    let seed_phrase = "SmartLegionLab_PCH_2026";
    let num_cities = 1000;

    println!("\nCONFIGURATION:");
    println!("  Cities:            {}", num_cities);
    println!("  Seed phrase:       '{}'", seed_phrase);

    let dg_cities = generate_cities(seed_phrase, num_cities);

    println!("\n[1/2] DYNAMIC GRAVITY (Initial Solution)");
    let start = Instant::now();
    let (initial_distance, initial_path) =
        dynamic_gravity_solve(&dg_cities, 0.9, true, 100, 0.3, true, 50);
    let initial_time = start.elapsed().as_secs_f64();
    println!("   Distance: {:.2}", initial_distance);
    println!("   Time: {:.3}s", initial_time);
    print_path_preview(&initial_path, "DYNAMIC GRAVITY");

    // Convert DG cities to DgCity (new type wrapper)
    let pch_cities: Vec<DgCity> = dg_cities.iter().map(|c| DgCity(*c)).collect();

    println!("\n[2/2] PCH IMPROVER");
    println!("   Parameters:");
    println!("     Permutations:        50");
    println!("     Batch size:          1000");
    println!("     Time limit:          30s");
    println!("     Top candidates:      300");
    println!("     Agents:              12");

    let start = Instant::now();
    let (improved_distance, improved_path) = pch_improve(
        &pch_cities,
        &initial_path,
        initial_distance,
        50,
        1000,
        30,
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
        println!("\n   ⚠️ No improvement found");
    }
    println!("================================================================================");
}
