//! Graph algorithms module.
//!
//! This module contains various graph algorithms implemented with safe, zero-cost abstractions.
//! All algorithms work with any type implementing the `Graph` trait.

/// Tarjan's strongly connected components algorithm.
pub mod tarjan;

pub use tarjan::tarjan;
