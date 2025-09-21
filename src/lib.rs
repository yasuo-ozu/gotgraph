#![doc = include_str!("../README.md")]

/// Graph algorithms module containing strongly connected components and other graph algorithms.
pub mod algo;
/// Core graph traits and context-based operations.
pub mod graph;
/// Vector-based graph implementation.
pub mod vec_graph;

/// Commonly used types and traits for easy importing.
///
/// This module re-exports the most frequently used items from the library,
/// allowing users to import everything they need with a single use statement.
///
/// # Example
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<i32, &str> = VecGraph::default();
/// // Now you have access to Graph, GraphUpdate, etc.
/// ```
pub mod prelude {
    pub use crate::graph::{Graph, GraphRemove, GraphRemoveEdge, GraphUpdate};
    pub use crate::vec_graph::VecGraph;
}

/// A trait for associative containers that map keys to values.
///
/// This trait provides a common interface for data structures that associate
/// keys with values, similar to hash maps or arrays. It's used extensively
/// throughout the library for creating mappings from node/edge indices to
/// computed values.
///
/// # Type Parameters
///
/// - `K`: The key type (typically node or edge indices)
/// - `V`: The value type
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<i32, &str> = VecGraph::default();
/// graph.scope_mut(|mut ctx| {
///     ctx.add_node(10);
///     ctx.add_node(20);
/// });
///
/// graph.scope(|ctx| {
///     // Create a mapping that doubles each node's value
///     let doubled = ctx.init_node_map(|_tag, &value| value * 2);
///     
///     for node_tag in ctx.node_indices() {
///         println!("Doubled: {}", doubled[node_tag]);
///     }
/// });
/// ```
pub trait Mapping<K, V>: std::ops::IndexMut<K, Output = V> + IntoIterator<Item = V> {
    /// Transform this mapping by applying a function to each value.
    ///
    /// Creates a new mapping with the same keys but transformed values.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that transforms each value from type `V` to type `VV`
    ///
    /// # Returns
    ///
    /// A new mapping with transformed values.
    fn map<VV>(self, f: impl FnMut(V) -> VV) -> impl Mapping<K, VV>;

    /// Returns an iterator over references to the values in this mapping.
    ///
    /// The order of iteration is implementation-defined.
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a V>
    where
        V: 'a;

    /// Returns an iterator over mutable references to the values in this mapping.
    ///
    /// The order of iteration is implementation-defined.
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V>
    where
        V: 'a;

    /// Gets a reference to the value associated with the given key without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the key exists in the mapping. Calling this method
    /// with a non-existent key results in undefined behavior.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to look up
    ///
    /// # Returns
    ///
    /// A reference to the value associated with the key.
    unsafe fn get_unchecked(&self, key: K) -> &V;

    /// Gets a mutable reference to the value associated with the given key without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the key exists in the mapping. Calling this method
    /// with a non-existent key results in undefined behavior.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to look up
    ///
    /// # Returns
    ///
    /// A mutable reference to the value associated with the key.
    unsafe fn get_unchecked_mut(&mut self, key: K) -> &mut V;
}

type Invariant<'a> = core::marker::PhantomData<fn(&'a ()) -> &'a ()>;
