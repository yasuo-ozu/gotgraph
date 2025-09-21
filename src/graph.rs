pub mod context;
pub mod remove;
pub mod update;

use crate::Mapping;
pub use context::{Context, EdgeTag, NodeTag};
pub use remove::{GraphRemove, GraphRemoveEdge};
pub use update::GraphUpdate;

/// The core trait defining the interface for all graph types.
///
/// This trait provides a comprehensive set of methods for working with graphs,
/// including node and edge access, traversal, and introspection. All graph
/// implementations must provide this interface.
///
/// # Type Parameters
///
/// - `Node`: The type of data stored in nodes
/// - `Edge`: The type of data stored in edges  
/// - `NodeIx`: The type used for node indices (must be copyable, debuggable, comparable, and hashable)
/// - `EdgeIx`: The type used for edge indices (must be copyable, debuggable, comparable, and hashable)
///
/// # Safety and Scope
///
/// Many methods in this trait have both checked and unchecked variants. The unchecked
/// variants are marked `unsafe` and require the caller to ensure that indices are valid.
/// Always prefer the checked variants unless performance is critical and you can guarantee
/// the safety requirements.
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<&str, i32> = VecGraph::default();
///
/// // Add some data and query within same scope
/// graph.scope_mut(|mut ctx| {
///     let n1 = ctx.add_node("Alice");
///     let n2 = ctx.add_node("Bob");
///     let e = ctx.add_edge(42, n1, n2);
///     
///     // Query the graph within the scope
///     println!("Added {} nodes", ctx.len_nodes());
///     println!("Added {} edges", ctx.len_edges());
/// });
/// ```
pub trait Graph {
    /// The type of data stored in each node.
    type Node;
    /// The type of data stored in each edge.
    type Edge;
    /// The type used for node indices. Must support copying, debugging, comparison, and hashing.
    type NodeIx: Copy + core::fmt::Debug + Eq + Ord + std::hash::Hash;
    /// The type used for edge indices. Must support copying, debugging, comparison, and hashing.
    type EdgeIx: Copy + core::fmt::Debug + Eq + Ord + std::hash::Hash;

    /// Checks whether a node index exists in the graph.
    ///
    /// # Parameters
    ///
    /// - `ix`: The node index to check
    ///
    /// # Returns
    ///
    /// `true` if the node exists, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gotgraph::prelude::*;
    ///
    /// let mut graph: VecGraph<i32, ()> = VecGraph::default();
    /// graph.scope_mut(|mut ctx| {
    ///     let node = ctx.add_node(42);
    ///     assert!(ctx.exists_node_index(node));
    /// });
    /// ```
    fn exists_node_index(&self, ix: Self::NodeIx) -> bool;

    /// Checks whether an edge index exists in the graph.
    ///
    /// # Parameters
    ///
    /// - `ix`: The edge index to check
    ///
    /// # Returns
    ///
    /// `true` if the edge exists, `false` otherwise.
    fn exists_edge_index(&self, ix: Self::EdgeIx) -> bool;

    /// Returns an iterator over all node indices in the graph.
    ///
    /// The order of iteration is implementation-defined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gotgraph::prelude::*;
    ///
    /// let mut graph: VecGraph<i32, ()> = VecGraph::default();
    /// graph.scope_mut(|mut ctx| {
    ///     ctx.add_node(1);
    ///     ctx.add_node(2);
    /// });
    ///
    /// let indices: Vec<_> = graph.node_indices().collect();
    /// assert_eq!(indices.len(), 2);
    /// ```
    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIx>;

    /// Returns an iterator over all edge indices in the graph.
    ///
    /// The order of iteration is implementation-defined.
    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIx>;
    /// Returns an iterator over the indices of edges originating from the specified node.
    ///
    /// This method includes bounds checking and will panic if the node index is invalid.
    ///
    /// # Parameters
    ///
    /// - `tag`: The node index to get outgoing edges for
    ///
    /// # Returns
    ///
    /// An iterator over the edge indices of outgoing edges.
    ///
    /// # Panics
    ///
    /// Panics if the node index does not exist in the graph.
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
    ///     ctx.add_edge("edge", n1, n2);
    ///     
    ///     let outgoing: Vec<_> = ctx.outgoing_edge_indices(n1).collect();
    ///     assert_eq!(outgoing.len(), 1);
    /// });
    /// ```
    fn outgoing_edge_indices(&self, tag: Self::NodeIx) -> impl Iterator<Item = Self::EdgeIx> {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.outgoing_edge_indices_unchecked(tag) }
    }

    /// Returns an iterator over the indices of edges originating from the specified node without bounds checking.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the node index is valid (i.e., `exists_node_index(tag)` returns `true`).
    /// Using an invalid node index results in undefined behavior.
    ///
    /// # Parameters
    ///
    /// - `tag`: The node index to get outgoing edges for
    ///
    /// # Returns
    ///
    /// An iterator over the edge indices of outgoing edges.
    unsafe fn outgoing_edge_indices_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        self.outgoing_edge_pairs_unchecked(tag).map(|(ix, _)| ix)
    }

    fn outgoing_edges(&self, tag: Self::NodeIx) -> impl Iterator<Item = &Self::Edge> {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.outgoing_edges_unchecked(tag) }
    }

    unsafe fn outgoing_edges_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &Self::Edge> {
        self.outgoing_edge_pairs_unchecked(tag)
            .map(|(_, edge)| edge)
    }

    fn outgoing_edge_pairs(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.outgoing_edge_pairs_unchecked(tag) }
    }

    unsafe fn outgoing_edge_pairs_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)>;

    fn incoming_edge_indices(&self, tag: Self::NodeIx) -> impl Iterator<Item = Self::EdgeIx> {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.incoming_edge_indices_unchecked(tag) }
    }

    unsafe fn incoming_edge_indices_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        self.incoming_edge_pairs_unchecked(tag).map(|(ix, _)| ix)
    }

    fn incoming_edges(&self, tag: Self::NodeIx) -> impl Iterator<Item = &Self::Edge> {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.incoming_edges_unchecked(tag) }
    }

    unsafe fn incoming_edges_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &Self::Edge> {
        self.incoming_edge_pairs_unchecked(tag)
            .map(|(_, edge)| edge)
    }

    fn incoming_edge_pairs(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.incoming_edge_pairs_unchecked(tag) }
    }

    unsafe fn incoming_edge_pairs_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)>;

    fn connecting_edge_indices(&self, tag: Self::NodeIx) -> impl Iterator<Item = Self::EdgeIx> {
        self.connecting_edge_pairs(tag).map(|(ix, _)| ix)
    }

    unsafe fn connecting_edge_indices_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        self.connecting_edge_pairs_unchecked(tag).map(|(ix, _)| ix)
    }

    fn connecting_edges(&self, tag: Self::NodeIx) -> impl Iterator<Item = &Self::Edge> {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.connecting_edges_unchecked(tag) }
    }

    unsafe fn connecting_edges_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &Self::Edge> {
        self.connecting_edge_pairs_unchecked(tag)
            .map(|(_, edge)| edge)
    }

    fn connecting_edge_pairs(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.connecting_edge_pairs_unchecked(tag) }
    }

    unsafe fn connecting_edge_pairs_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        self.outgoing_edge_pairs_unchecked(tag)
            .chain(self.incoming_edge_pairs_unchecked(tag))
    }

    fn node(&self, tag: Self::NodeIx) -> &Self::Node {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.node_unchecked(tag) }
    }

    unsafe fn node_unchecked(&self, tag: Self::NodeIx) -> &Self::Node;

    fn edge(&self, tag: Self::EdgeIx) -> &Self::Edge {
        assert!(
            self.exists_edge_index(tag),
            "Edge index {:?} does not exist",
            tag
        );
        unsafe { self.edge_unchecked(tag) }
    }

    unsafe fn edge_unchecked(&self, tag: Self::EdgeIx) -> &Self::Edge;

    fn endpoints(&self, tag: Self::EdgeIx) -> [Self::NodeIx; 2] {
        assert!(
            self.exists_edge_index(tag),
            "Edge index {:?} does not exist",
            tag
        );
        unsafe { self.endpoints_unchecked(tag) }
    }

    unsafe fn endpoints_unchecked(&self, ix: Self::EdgeIx) -> [Self::NodeIx; 2];

    fn nodes(&self) -> impl Iterator<Item = &Self::Node> {
        self.node_pairs().map(|(_, node)| node)
    }

    fn edges(&self) -> impl Iterator<Item = &Self::Edge> + use<'_, Self> {
        self.edge_pairs().map(|(_, edge)| edge)
    }

    fn node_pairs(&self) -> impl Iterator<Item = (Self::NodeIx, &Self::Node)> {
        self.node_indices()
            .map(move |node_ix| (node_ix, unsafe { self.node_unchecked(node_ix) }))
    }

    fn edge_pairs(&self) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        self.edge_indices()
            .map(move |edge_ix| (edge_ix, unsafe { self.edge_unchecked(edge_ix) }))
    }

    fn len_nodes(&self) -> usize {
        self.node_indices().count()
    }
    fn len_edges(&self) -> usize {
        self.edge_indices().count()
    }
    fn is_empty(&self) -> bool {
        self.len_nodes() == 0 && self.len_edges() == 0
    }

    fn scope<
        'graph,
        R,
        F: for<'scope> FnOnce(&crate::graph::context::Context<'scope, &'graph Self>) -> R,
    >(
        &'graph self,
        f: F,
    ) -> R {
        use core::marker::PhantomData;
        f(&crate::graph::context::Context {
            graph: self,
            _scope: PhantomData,
        })
    }

    fn node_mut(&mut self, tag: Self::NodeIx) -> &mut Self::Node {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.node_unchecked_mut(tag) }
    }

    unsafe fn node_unchecked_mut(&mut self, tag: Self::NodeIx) -> &mut Self::Node;

    fn edge_mut(&mut self, tag: Self::EdgeIx) -> &mut Self::Edge {
        assert!(
            self.exists_edge_index(tag),
            "Edge index {:?} does not exist",
            tag
        );
        unsafe { self.edge_unchecked_mut(tag) }
    }

    unsafe fn edge_unchecked_mut(&mut self, tag: Self::EdgeIx) -> &mut Self::Edge;

    fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Self::Node> + use<'_, Self>
    where
        Self: Sized,
    {
        self.node_pairs_mut().map(|(_, node)| node)
    }

    fn edges_mut(&mut self) -> impl Iterator<Item = &mut Self::Edge> + use<'_, Self>
    where
        Self: Sized,
    {
        self.edge_pairs_mut().map(|(_, edge)| edge)
    }

    fn node_pairs_mut(
        &mut self,
    ) -> impl Iterator<Item = (Self::NodeIx, &mut Self::Node)> + use<'_, Self>
    where
        Self: Sized,
    {
        struct NodePairsMutIter<'a, G: Graph> {
            graph: &'a mut G,
            indices: std::vec::IntoIter<G::NodeIx>,
        }

        impl<'a, G: Graph> Iterator for NodePairsMutIter<'a, G> {
            type Item = (G::NodeIx, &'a mut G::Node);

            fn next(&mut self) -> Option<Self::Item> {
                self.indices.next().map(|ix| unsafe {
                    let ptr = self.graph.node_unchecked_mut(ix) as *mut G::Node;
                    (ix, &mut *ptr)
                })
            }
        }

        let indices: Vec<_> = self.node_indices().collect();
        NodePairsMutIter {
            graph: self,
            indices: indices.into_iter(),
        }
    }

    fn edge_pairs_mut(
        &mut self,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)> + use<'_, Self>
    where
        Self: Sized,
    {
        struct EdgePairsMutIter<'a, G: Graph> {
            graph: &'a mut G,
            indices: std::vec::IntoIter<G::EdgeIx>,
        }

        impl<'a, G: Graph> Iterator for EdgePairsMutIter<'a, G> {
            type Item = (G::EdgeIx, &'a mut G::Edge);

            fn next(&mut self) -> Option<Self::Item> {
                self.indices.next().map(|ix| unsafe {
                    let ptr = self.graph.edge_unchecked_mut(ix) as *mut G::Edge;
                    (ix, &mut *ptr)
                })
            }
        }

        let indices: Vec<_> = self.edge_indices().collect();
        EdgePairsMutIter {
            graph: self,
            indices: indices.into_iter(),
        }
    }

    fn outgoing_edges_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &mut Self::Edge> + use<'_, Self>
    where
        Self: Sized,
    {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.outgoing_edges_unchecked_mut(tag) }
    }

    unsafe fn outgoing_edges_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &mut Self::Edge> + use<'_, Self>
    where
        Self: Sized,
    {
        self.outgoing_edge_pairs_unchecked_mut(tag)
            .map(|(_, edge)| edge)
    }

    fn outgoing_edge_pairs_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)> + use<'_, Self>
    where
        Self: Sized,
    {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.outgoing_edge_pairs_unchecked_mut(tag) }
    }

    unsafe fn outgoing_edge_pairs_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized;

    fn incoming_edges_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &mut Self::Edge> + use<'_, Self>
    where
        Self: Sized,
    {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.incoming_edges_unchecked_mut(tag) }
    }

    unsafe fn incoming_edges_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &mut Self::Edge> + use<'_, Self>
    where
        Self: Sized,
    {
        self.incoming_edge_pairs_unchecked_mut(tag)
            .map(|(_, edge)| edge)
    }

    fn incoming_edge_pairs_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)> + use<'_, Self>
    where
        Self: Sized,
    {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.incoming_edge_pairs_unchecked_mut(tag) }
    }

    unsafe fn incoming_edge_pairs_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized;

    fn connecting_edges_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &mut Self::Edge> + use<'_, Self>
    where
        Self: Sized,
    {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.connecting_edges_unchecked_mut(tag) }
    }

    unsafe fn connecting_edges_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = &mut Self::Edge> + use<'_, Self>
    where
        Self: Sized,
    {
        self.connecting_edge_pairs_unchecked_mut(tag)
            .map(|(_, edge)| edge)
    }

    fn connecting_edge_pairs_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)> + use<'_, Self>
    where
        Self: Sized,
    {
        assert!(
            self.exists_node_index(tag),
            "Node index {:?} does not exist",
            tag
        );
        unsafe { self.connecting_edge_pairs_unchecked_mut(tag) }
    }

    unsafe fn connecting_edge_pairs_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized;

    fn scope_mut<
        'graph,
        R,
        F: for<'scope> FnOnce(crate::graph::context::Context<'scope, &'graph mut Self>) -> R,
    >(
        &'graph mut self,
        f: F,
    ) -> R
    where
        Self: Sized + crate::graph::GraphUpdate,
    {
        use core::marker::PhantomData;
        f(crate::graph::context::Context {
            graph: self,
            _scope: PhantomData,
        })
    }

    fn init_edge_map<V>(
        &self,
        mut f: impl FnMut(Self::EdgeIx, &Self::Edge) -> V,
    ) -> impl Mapping<Self::EdgeIx, V> {
        #[derive(Debug)]
        struct DefaultEdgeMap<K, V>(std::collections::HashMap<K, V>);

        impl<K: Eq + std::hash::Hash, V> std::ops::Index<K> for DefaultEdgeMap<K, V> {
            type Output = V;

            fn index(&self, key: K) -> &Self::Output {
                &self.0[&key]
            }
        }

        impl<K: Eq + std::hash::Hash, V> std::ops::IndexMut<K> for DefaultEdgeMap<K, V> {
            fn index_mut(&mut self, key: K) -> &mut Self::Output {
                self.0.get_mut(&key).expect("Key not found in mapping")
            }
        }

        impl<K: Eq + std::hash::Hash, V> IntoIterator for DefaultEdgeMap<K, V> {
            type Item = V;
            type IntoIter = std::collections::hash_map::IntoValues<K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_values()
            }
        }

        impl<K: Eq + std::hash::Hash, V> Mapping<K, V> for DefaultEdgeMap<K, V> {
            fn map<VV>(self, mut f: impl FnMut(V) -> VV) -> impl Mapping<K, VV> {
                DefaultEdgeMap(
                    self.0
                        .into_iter()
                        .map(|(k, v)| (k, f(v)))
                        .collect::<std::collections::HashMap<K, VV>>(),
                )
            }

            fn iter<'a>(&'a self) -> impl Iterator<Item = &'a V>
            where
                V: 'a,
            {
                self.0.values()
            }

            fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V>
            where
                V: 'a,
            {
                self.0.values_mut()
            }

            unsafe fn get_unchecked(&self, key: K) -> &V {
                self.0.get(&key).unwrap_unchecked()
            }

            unsafe fn get_unchecked_mut(&mut self, key: K) -> &mut V {
                self.0.get_mut(&key).unwrap_unchecked()
            }
        }

        let mut map = std::collections::HashMap::new();
        for (edge_ix, edge) in self.edge_pairs() {
            map.insert(edge_ix, f(edge_ix, edge));
        }
        DefaultEdgeMap(map)
    }

    fn init_node_map<V>(
        &self,
        mut f: impl FnMut(Self::NodeIx, &Self::Node) -> V,
    ) -> impl Mapping<Self::NodeIx, V> {
        #[derive(Debug)]
        struct DefaultNodeMap<K, V>(std::collections::HashMap<K, V>);

        impl<K: Eq + std::hash::Hash, V> std::ops::Index<K> for DefaultNodeMap<K, V> {
            type Output = V;

            fn index(&self, key: K) -> &Self::Output {
                &self.0[&key]
            }
        }

        impl<K: Eq + std::hash::Hash, V> std::ops::IndexMut<K> for DefaultNodeMap<K, V> {
            fn index_mut(&mut self, key: K) -> &mut Self::Output {
                self.0.get_mut(&key).expect("Key not found in mapping")
            }
        }

        impl<K: Eq + std::hash::Hash, V> IntoIterator for DefaultNodeMap<K, V> {
            type Item = V;
            type IntoIter = std::collections::hash_map::IntoValues<K, V>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_values()
            }
        }

        impl<K: Eq + std::hash::Hash, V> Mapping<K, V> for DefaultNodeMap<K, V> {
            fn map<VV>(self, mut f: impl FnMut(V) -> VV) -> impl Mapping<K, VV> {
                DefaultNodeMap(
                    self.0
                        .into_iter()
                        .map(|(k, v)| (k, f(v)))
                        .collect::<std::collections::HashMap<K, VV>>(),
                )
            }

            fn iter<'a>(&'a self) -> impl Iterator<Item = &'a V>
            where
                V: 'a,
            {
                self.0.values()
            }

            fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V>
            where
                V: 'a,
            {
                self.0.values_mut()
            }

            unsafe fn get_unchecked(&self, key: K) -> &V {
                self.0.get(&key).unwrap_unchecked()
            }

            unsafe fn get_unchecked_mut(&mut self, key: K) -> &mut V {
                self.0.get_mut(&key).unwrap_unchecked()
            }
        }

        let mut map = std::collections::HashMap::new();
        for (node_ix, node) in self.node_pairs() {
            map.insert(node_ix, f(node_ix, node));
        }
        DefaultNodeMap(map)
    }
}

impl<T: Graph> Graph for &T {
    type Node = T::Node;
    type Edge = T::Edge;
    type NodeIx = T::NodeIx;
    type EdgeIx = T::EdgeIx;

    fn exists_node_index(&self, ix: Self::NodeIx) -> bool {
        (*self).exists_node_index(ix)
    }

    fn exists_edge_index(&self, ix: Self::EdgeIx) -> bool {
        (*self).exists_edge_index(ix)
    }

    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIx> {
        (*self).node_indices()
    }

    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIx> {
        (*self).edge_indices()
    }

    fn outgoing_edge_indices(&self, tag: Self::NodeIx) -> impl Iterator<Item = Self::EdgeIx> {
        (*self).outgoing_edge_indices(tag)
    }

    unsafe fn outgoing_edge_indices_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        (*self).outgoing_edge_indices_unchecked(tag)
    }

    fn incoming_edge_indices(&self, tag: Self::NodeIx) -> impl Iterator<Item = Self::EdgeIx> {
        (*self).incoming_edge_indices(tag)
    }

    unsafe fn incoming_edge_indices_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        (*self).incoming_edge_indices_unchecked(tag)
    }

    fn node(&self, tag: Self::NodeIx) -> &Self::Node {
        (*self).node(tag)
    }

    unsafe fn node_unchecked(&self, tag: Self::NodeIx) -> &Self::Node {
        (*self).node_unchecked(tag)
    }

    fn edge(&self, tag: Self::EdgeIx) -> &Self::Edge {
        (*self).edge(tag)
    }

    unsafe fn edge_unchecked(&self, tag: Self::EdgeIx) -> &Self::Edge {
        (*self).edge_unchecked(tag)
    }

    fn endpoints(&self, tag: Self::EdgeIx) -> [Self::NodeIx; 2] {
        (*self).endpoints(tag)
    }

    unsafe fn endpoints_unchecked(&self, ix: Self::EdgeIx) -> [Self::NodeIx; 2] {
        (*self).endpoints_unchecked(ix)
    }

    unsafe fn outgoing_edge_pairs_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        (*self).outgoing_edge_pairs_unchecked(tag)
    }

    unsafe fn incoming_edge_pairs_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        (*self).incoming_edge_pairs_unchecked(tag)
    }

    unsafe fn node_unchecked_mut(&mut self, _tag: Self::NodeIx) -> &mut Self::Node {
        panic!("&T does not support mutable access")
    }

    unsafe fn edge_unchecked_mut(&mut self, _tag: Self::EdgeIx) -> &mut Self::Edge {
        panic!("&T does not support mutable access")
    }

    unsafe fn outgoing_edge_pairs_unchecked_mut(
        &mut self,
        _tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        std::iter::empty()
    }

    unsafe fn incoming_edge_pairs_unchecked_mut(
        &mut self,
        _tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        std::iter::empty()
    }

    unsafe fn connecting_edge_pairs_unchecked_mut(
        &mut self,
        _tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        std::iter::empty()
    }
}

impl<T: Graph> Graph for &mut T {
    type Node = T::Node;
    type Edge = T::Edge;
    type NodeIx = T::NodeIx;
    type EdgeIx = T::EdgeIx;

    fn exists_node_index(&self, ix: Self::NodeIx) -> bool {
        (**self).exists_node_index(ix)
    }

    fn exists_edge_index(&self, ix: Self::EdgeIx) -> bool {
        (**self).exists_edge_index(ix)
    }

    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIx> {
        (**self).node_indices()
    }

    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIx> {
        (**self).edge_indices()
    }

    fn outgoing_edge_indices(&self, tag: Self::NodeIx) -> impl Iterator<Item = Self::EdgeIx> {
        (**self).outgoing_edge_indices(tag)
    }

    unsafe fn outgoing_edge_indices_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        (**self).outgoing_edge_indices_unchecked(tag)
    }

    fn incoming_edge_indices(&self, tag: Self::NodeIx) -> impl Iterator<Item = Self::EdgeIx> {
        (**self).incoming_edge_indices(tag)
    }

    unsafe fn incoming_edge_indices_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        (**self).incoming_edge_indices_unchecked(tag)
    }

    fn node(&self, tag: Self::NodeIx) -> &Self::Node {
        (**self).node(tag)
    }

    unsafe fn node_unchecked(&self, tag: Self::NodeIx) -> &Self::Node {
        (**self).node_unchecked(tag)
    }

    fn edge(&self, tag: Self::EdgeIx) -> &Self::Edge {
        (**self).edge(tag)
    }

    unsafe fn edge_unchecked(&self, tag: Self::EdgeIx) -> &Self::Edge {
        (**self).edge_unchecked(tag)
    }

    fn endpoints(&self, tag: Self::EdgeIx) -> [Self::NodeIx; 2] {
        (**self).endpoints(tag)
    }

    unsafe fn endpoints_unchecked(&self, ix: Self::EdgeIx) -> [Self::NodeIx; 2] {
        (**self).endpoints_unchecked(ix)
    }

    unsafe fn outgoing_edge_pairs_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        (**self).outgoing_edge_pairs_unchecked(tag)
    }

    unsafe fn incoming_edge_pairs_unchecked(
        &self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        (**self).incoming_edge_pairs_unchecked(tag)
    }

    unsafe fn node_unchecked_mut(&mut self, tag: Self::NodeIx) -> &mut Self::Node {
        (**self).node_unchecked_mut(tag)
    }

    unsafe fn edge_unchecked_mut(&mut self, tag: Self::EdgeIx) -> &mut Self::Edge {
        (**self).edge_unchecked_mut(tag)
    }

    unsafe fn outgoing_edge_pairs_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        (**self).outgoing_edge_pairs_unchecked_mut(tag)
    }

    unsafe fn incoming_edge_pairs_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        (**self).incoming_edge_pairs_unchecked_mut(tag)
    }

    unsafe fn connecting_edge_pairs_unchecked_mut(
        &mut self,
        tag: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        (**self).connecting_edge_pairs_unchecked_mut(tag)
    }
}
