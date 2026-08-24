//! Universal TSP path improver using the PCH (Position-Candidate-Hypothesis) paradigm.
//!
//! PCH can improve ANY existing TSP path from ANY algorithm.
//! Works with any type that implements the `Point` trait.

pub mod algorithms;
pub mod core;

pub use algorithms::pch_improve;
pub use core::{City, Point};
