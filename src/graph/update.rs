use super::Graph;

/// Trait for graphs that support adding nodes and edges.
///
/// This trait extends the base `Graph` trait with mutation operations for adding
/// new nodes and edges to the graph. It provides both checked and unchecked variants
/// for performance-critical scenarios.
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<i32, &str> = VecGraph::default();
///
/// graph.scope_mut(|mut ctx| {
///     let node1 = ctx.add_node(42);
///     let node2 = ctx.add_node(100);
///     let edge = ctx.add_edge("connects", node1, node2);
///     
///     println!("Added edge between {} and {}",
///              ctx.node(node1), ctx.node(node2));
/// });
/// ```
pub trait GraphUpdate: Graph {
    /// Adds a new node to the graph with the given data.
    ///
    /// # Parameters
    ///
    /// - `node`: The data to store in the new node
    ///
    /// # Returns
    ///
    /// The index of the newly created node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gotgraph::prelude::*;
    ///
    /// let mut graph: VecGraph<&str, ()> = VecGraph::default();
    /// graph.scope_mut(|mut ctx| {
    ///     let node = ctx.add_node("Alice");
    /// });
    /// ```
    fn add_node(&mut self, node: Self::Node) -> Self::NodeIx;

    /// Adds a new edge to the graph between two nodes.
    ///
    /// This method includes bounds checking to ensure both endpoint nodes exist.
    ///
    /// # Parameters
    ///
    /// - `edge`: The data to store in the new edge
    /// - `from`: The source node index
    /// - `to`: The target node index
    ///
    /// # Returns
    ///
    /// The index of the newly created edge.
    ///
    /// # Panics
    ///
    /// Panics if either `from` or `to` node indices don't exist in the graph.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gotgraph::prelude::*;
    ///
    /// let mut graph: VecGraph<i32, &str> = VecGraph::default();
    /// graph.scope_mut(|mut ctx| {
    ///     let n1 = ctx.add_node(1);
    ///     let n2 = ctx.add_node(2);
    ///     let edge = ctx.add_edge("connection", n1, n2);
    /// });
    /// ```
    fn add_edge(&mut self, edge: Self::Edge, from: Self::NodeIx, to: Self::NodeIx) -> Self::EdgeIx {
        assert!(self.exists_node_index(from));
        assert!(self.exists_node_index(to));
        unsafe { self.add_edge_unchecked(edge, from, to) }
    }

    /// Adds a new edge to the graph between two nodes without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that both `from` and `to` node indices exist in the graph
    /// (i.e., `exists_node_index(from)` and `exists_node_index(to)` both return `true`).
    /// Using invalid node indices results in undefined behavior.
    ///
    /// # Parameters
    ///
    /// - `edge`: The data to store in the new edge
    /// - `from`: The source node index
    /// - `to`: The target node index
    ///
    /// # Returns
    ///
    /// The index of the newly created edge.
    unsafe fn add_edge_unchecked(
        &mut self,
        edge: Self::Edge,
        from: Self::NodeIx,
        to: Self::NodeIx,
    ) -> Self::EdgeIx {
        self.add_edge(edge, from, to)
    }

    fn append<G>(&mut self, mut other: G)
    where
        Self: Sized,
        G: GraphUpdate<Node = Self::Node, Edge = Self::Edge>,
        G: crate::graph::GraphRemove,
    {
        use std::collections::HashMap;

        // Collect all indices and their data before draining
        let edge_data: Vec<_> = other
            .edge_indices()
            .map(|edge_ix| {
                let endpoints = unsafe { other.endpoints_unchecked(edge_ix) };
                (edge_ix, endpoints)
            })
            .collect();

        let node_indices: Vec<_> = other.node_indices().collect();

        // Drain all nodes and edges at once to avoid index invalidation
        let (nodes, edges): (Vec<Self::Node>, Vec<Self::Edge>) = other.drain();

        // Create mapping from old node indices to new node indices
        let mut node_mapping = HashMap::new();

        // Add all nodes and build mapping
        for (old_node_ix, node) in node_indices.into_iter().zip(nodes) {
            let new_node_ix = self.add_node(node);
            node_mapping.insert(old_node_ix, new_node_ix);
        }

        // Add edges with mapped node indices
        for ((_, endpoints), edge) in edge_data.into_iter().zip(edges) {
            let new_from = node_mapping[&endpoints[0]];
            let new_to = node_mapping[&endpoints[1]];
            unsafe { self.add_edge_unchecked(edge, new_from, new_to) };
        }
    }
}

impl<T: GraphUpdate> GraphUpdate for &mut T {
    fn add_node(&mut self, node: Self::Node) -> Self::NodeIx {
        (**self).add_node(node)
    }

    fn add_edge(&mut self, edge: Self::Edge, from: Self::NodeIx, to: Self::NodeIx) -> Self::EdgeIx {
        (**self).add_edge(edge, from, to)
    }

    unsafe fn add_edge_unchecked(
        &mut self,
        edge: Self::Edge,
        from: Self::NodeIx,
        to: Self::NodeIx,
    ) -> Self::EdgeIx {
        (**self).add_edge_unchecked(edge, from, to)
    }

    fn append<G>(&mut self, other: G)
    where
        Self: Sized,
        G: GraphUpdate<Node = Self::Node, Edge = Self::Edge>,
        G: crate::graph::GraphRemove,
    {
        (**self).append(other)
    }
}
