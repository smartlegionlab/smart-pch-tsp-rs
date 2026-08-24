use criterion::{Criterion, criterion_group, criterion_main};
use smart_pch_tsp::{City, pch_improve};

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

fn benchmark_pch_improve(c: &mut Criterion) {
    let sizes = [100, 500, 1000, 2000];

    for &size in &sizes {
        let cities = generate_cities(size);
        let initial_path: Vec<usize> = (0..size).chain(std::iter::once(0)).collect();
        let initial_distance = 0.0;

        c.bench_function(&format!("pch_improve_{}", size), |b| {
            b.iter(|| {
                let _ = pch_improve(
                    &cities,
                    &initial_path,
                    initial_distance,
                    50,
                    1000,
                    30,
                    300,
                    12,
                );
            })
        });
    }
}

criterion_group!(benches, benchmark_pch_improve);
criterion_main!(benches);
