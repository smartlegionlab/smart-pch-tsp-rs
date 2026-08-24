//! Distance calculation utilities.

use super::point::Point;

/// Calculates the total distance of a cycle path through points.
pub fn calculate_cycle_distance<P: Point>(path: &[usize], points: &[P]) -> f64 {
    if path.is_empty() {
        return f64::MAX;
    }
    let mut total = 0.0;

    for i in 0..path.len() - 1 {
        let p1 = &points[path[i]];
        let p2 = &points[path[i + 1]];
        let dx = p1.x() - p2.x();
        let dy = p1.y() - p2.y();
        total += (dx * dx + dy * dy).sqrt();
    }
    total
}

/// Computes a symmetric distance matrix for all point pairs.
pub fn compute_dist_matrix<P: Point>(points: &[P]) -> Vec<Vec<f64>> {
    let n = points.len();
    let mut matrix = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in i..n {
            let dx = points[i].x() - points[j].x();
            let dy = points[i].y() - points[j].y();
            let dist = (dx * dx + dy * dy).sqrt();
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }
    matrix
}
