use smart_pch_tsp::{City, pch_improve};

#[test]
fn test_pch_improve_small() {
    let points = vec![
        City { x: 0.0, y: 0.0 },
        City { x: 1.0, y: 0.0 },
        City { x: 0.0, y: 1.0 },
        City { x: 1.0, y: 1.0 },
    ];
    let initial_path = vec![0, 1, 2, 3, 0];
    let initial_distance = 4.0;

    let (improved_distance, improved_path) =
        pch_improve(&points, &initial_path, initial_distance, 10, 100, 5, 10, 2);

    assert!(improved_distance <= initial_distance);
    assert_eq!(improved_path.len(), points.len() + 1);
    assert_eq!(improved_path[0], improved_path[improved_path.len() - 1]);
}

#[test]
fn test_empty_points() {
    let points: Vec<City> = vec![];
    let initial_path: Vec<usize> = vec![];
    let (distance, path) = pch_improve(&points, &initial_path, 0.0, 50, 1000, 30, 300, 12);
    assert_eq!(distance, 0.0);
    assert!(path.is_empty());
}

#[test]
fn test_single_city() {
    let points = vec![City { x: 0.0, y: 0.0 }];
    let initial_path = vec![0, 0];
    let (distance, path) = pch_improve(&points, &initial_path, 0.0, 50, 1000, 30, 300, 12);
    assert_eq!(distance, 0.0);
    assert_eq!(path, vec![0, 0]);
}
