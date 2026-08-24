//! Trait for generic point types.

/// Trait for types that represent a point in 2D space.
///
/// This allows PCH to work with any point type that provides x and y coordinates.
pub trait Point: Clone + Send + Sync + 'static {
    /// Returns the x-coordinate of the point.
    fn x(&self) -> f64;
    /// Returns the y-coordinate of the point.
    fn y(&self) -> f64;
}
