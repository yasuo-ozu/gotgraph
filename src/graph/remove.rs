use super::{update::GraphUpdate, Graph};

/// Trait for graphs that support removing edges.
///
/// This trait provides methods for removing individual edges or batches of edges
/// from a graph. It includes both checked and unchecked variants for performance.
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<i32, &str> = VecGraph::default();
///
/// graph.scope_mut(|mut ctx| {
///     let n1 = ctx.add_node(1);
///     let n2 = ctx.add_node(2);
///     let edge = ctx.add_edge("connection", n1, n2);
///     
///     // Remove the edge within the same scope
///     let (nodes, edges): (Vec<i32>, Vec<&str>) = ctx.remove_nodes_edges([], [edge]);
///     println!("Removed edge: {:?}", edges[0]);
/// });
/// ```
pub trait GraphRemoveEdge: Graph {
    /// Removes an edge from the graph and returns its data.
    ///
    /// This method includes bounds checking and will panic if the edge index is invalid.
    ///
    /// # Parameters
    ///
    /// - `ix`: The edge index to remove
    ///
    /// # Returns
    ///
    /// The data that was stored in the removed edge.
    ///
    /// # Panics
    ///
    /// Panics if the edge index does not exist in the graph.
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
    ///     let edge = ctx.add_edge("test", n1, n2);
    ///     
    ///     // Remove the edge within the same scope
    ///     let (_, edges): (Vec<i32>, Vec<&str>) = ctx.remove_nodes_edges([], [edge]);
    ///     assert_eq!(edges[0], "test");
    /// });
    /// ```
    fn remove_edge(&mut self, ix: Self::EdgeIx) -> Self::Edge {
        assert!(
            self.exists_edge_index(ix),
            "Edge index {:?} does not exist",
            ix
        );
        unsafe { self.remove_edge_unchecked(ix) }
    }

    /// Removes an edge from the graph without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the edge index is valid (i.e., `exists_edge_index(ix)` returns `true`).
    /// Using an invalid edge index results in undefined behavior.
    ///
    /// # Parameters
    ///
    /// - `ix`: The edge index to remove
    ///
    /// # Returns
    ///
    /// The data that was stored in the removed edge.
    unsafe fn remove_edge_unchecked(&mut self, ix: Self::EdgeIx) -> Self::Edge;

    fn clear_edges(&mut self)
    where
        Self: Sized,
    {
        let edges: Vec<_> = self.edge_indices().collect();
        for edge_ix in edges {
            if self.exists_edge_index(edge_ix) {
                self.remove_edge(edge_ix);
            }
        }
    }

    fn remove_edges_with<F: FnMut(&Self::Edge) -> bool>(
        &mut self,
        mut f: F,
    ) -> impl Iterator<Item = Self::Edge> + use<'_, Self, F> {
        let to_remove: Vec<_> = self
            .edge_indices()
            .filter(|&ix| unsafe { f(self.edge_unchecked(ix)) })
            .collect();

        to_remove.into_iter().filter_map(move |ix| {
            if self.exists_edge_index(ix) {
                Some(self.remove_edge(ix))
            } else {
                None
            }
        })
    }
}

pub trait GraphRemove: GraphUpdate + GraphRemoveEdge {
    fn remove_node(&mut self, ix: Self::NodeIx) -> Self::Node {
        assert!(
            self.exists_node_index(ix),
            "Node index {:?} does not exist",
            ix
        );
        unsafe { self.remove_node_unchecked(ix) }
    }

    unsafe fn remove_node_unchecked(&mut self, ix: Self::NodeIx) -> Self::Node {
        self.remove_node(ix)
    }

    fn remove_nodes_edges<CN, CE>(
        &mut self,
        nodes: impl IntoIterator<Item = Self::NodeIx>,
        edges: impl IntoIterator<Item = Self::EdgeIx>,
    ) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>,
    {
        let mut removed_nodes = CN::default();
        let mut removed_edges = CE::default();

        for edge_ix in edges {
            if self.exists_edge_index(edge_ix) {
                removed_edges.extend(std::iter::once(unsafe {
                    self.remove_edge_unchecked(edge_ix)
                }));
            }
        }

        for node_ix in nodes {
            if self.exists_node_index(node_ix) {
                removed_nodes.extend(std::iter::once(unsafe {
                    self.remove_node_unchecked(node_ix)
                }));
            }
        }

        (removed_nodes, removed_edges)
    }

    unsafe fn remove_nodes_edges_unchecked<CN, CE>(
        &mut self,
        nodes: impl IntoIterator<Item = Self::NodeIx>,
        edges: impl IntoIterator<Item = Self::EdgeIx>,
    ) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>,
        Self: Sized,
    {
        self.remove_nodes_edges(nodes, edges)
    }

    fn drain<CN, CE>(&mut self) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>,
        Self: Sized,
    {
        let nodes: Vec<_> = self.node_indices().collect();
        let edges: Vec<_> = self.edge_indices().collect();
        unsafe { self.remove_nodes_edges_unchecked(nodes, edges) }
    }

    fn clear(&mut self)
    where
        Self: Sized,
    {
        let _: (Vec<Self::Node>, Vec<Self::Edge>) = self.drain();
    }

    fn remove_nodes_with<F: FnMut(&Self::Node) -> bool>(
        &mut self,
        mut f: F,
    ) -> impl Iterator<Item = Self::Node> + use<'_, Self, F> {
        let to_remove: Vec<_> = self
            .node_indices()
            .filter(|&ix| unsafe { f(self.node_unchecked(ix)) })
            .collect();

        to_remove.into_iter().filter_map(move |ix| {
            if self.exists_node_index(ix) {
                Some(self.remove_node(ix))
            } else {
                None
            }
        })
    }
}

impl<T: GraphRemoveEdge> GraphRemoveEdge for &mut T {
    unsafe fn remove_edge_unchecked(&mut self, ix: Self::EdgeIx) -> Self::Edge {
        (**self).remove_edge_unchecked(ix)
    }
}

impl<T: GraphRemove> GraphRemove for &mut T {
    fn remove_node(&mut self, ix: Self::NodeIx) -> Self::Node {
        (**self).remove_node(ix)
    }

    unsafe fn remove_node_unchecked(&mut self, ix: Self::NodeIx) -> Self::Node {
        (**self).remove_node_unchecked(ix)
    }

    fn remove_nodes_edges<CN, CE>(
        &mut self,
        nodes: impl IntoIterator<Item = Self::NodeIx>,
        edges: impl IntoIterator<Item = Self::EdgeIx>,
    ) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>,
    {
        (**self).remove_nodes_edges(nodes, edges)
    }

    unsafe fn remove_nodes_edges_unchecked<CN, CE>(
        &mut self,
        nodes: impl IntoIterator<Item = Self::NodeIx>,
        edges: impl IntoIterator<Item = Self::EdgeIx>,
    ) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>,
        Self: Sized,
    {
        (**self).remove_nodes_edges_unchecked(nodes, edges)
    }
}
