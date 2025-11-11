use core::marker::PhantomData;

use crate::graph::{Graph, GraphRemove, GraphUpdate};

/// A scoped mapping from node tags to values.
///
/// This type ensures that mappings created within a scope cannot escape that scope,
/// preventing use-after-remove bugs and cross-graph contamination. The mapping
/// can only be indexed with `NodeTag`s from the same scope.
///
/// # Type Parameters
///
/// - `'scope`: The lifetime of the scope this mapping is valid in
/// - `K`: The underlying key type (typically a node index)
/// - `V`: The value type stored in the mapping
/// - `M`: The underlying mapping implementation
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<i32, ()> = VecGraph::default();
/// graph.scope_mut(|mut ctx| {
///     let n1 = ctx.add_node(10);
///     let n2 = ctx.add_node(20);
///     
///     let node_map = ctx.init_node_map(|_tag, &value| value * 2);
///     println!("Doubled value: {}", node_map[n1]);
/// });
/// ```
#[derive(Debug)]
pub struct ContextNodeMap<'scope, K, V, M> {
    _scope: crate::Invariant<'scope>,
    _key: core::marker::PhantomData<K>,
    _value: core::marker::PhantomData<V>,
    inner: M,
}

/// A scoped mapping from edge tags to values.
///
/// This type ensures that mappings created within a scope cannot escape that scope,
/// preventing use-after-remove bugs and cross-graph contamination. The mapping
/// can only be indexed with `EdgeTag`s from the same scope.
///
/// # Type Parameters
///
/// - `'scope`: The lifetime of the scope this mapping is valid in
/// - `K`: The underlying key type (typically an edge index)
/// - `V`: The value type stored in the mapping
/// - `M`: The underlying mapping implementation
#[derive(Debug)]
pub struct ContextEdgeMap<'scope, K, V, M> {
    _scope: crate::Invariant<'scope>,
    _key: core::marker::PhantomData<K>,
    _value: core::marker::PhantomData<V>,
    inner: M,
}

macro_rules! impl_context_map {
    ($map_type:ident, $tag_type:ident) => {
        impl<'scope, K, V, M: crate::Mapping<K, V>> std::ops::Index<$tag_type<'scope, K>>
            for $map_type<'scope, K, V, M>
        {
            type Output = V;

            fn index(&self, $tag_type(_, ix): $tag_type<'scope, K>) -> &Self::Output {
                unsafe { self.inner.get_unchecked(ix) }
            }
        }

        impl<'scope, K, V, M: crate::Mapping<K, V>> std::ops::IndexMut<$tag_type<'scope, K>>
            for $map_type<'scope, K, V, M>
        {
            fn index_mut(&mut self, $tag_type(_, ix): $tag_type<'scope, K>) -> &mut Self::Output {
                unsafe { self.inner.get_unchecked_mut(ix) }
            }
        }

        impl<'scope, K, V, M: crate::Mapping<K, V>> IntoIterator for $map_type<'scope, K, V, M> {
            type Item = V;
            type IntoIter = M::IntoIter;

            fn into_iter(self) -> Self::IntoIter {
                self.inner.into_iter()
            }
        }

        impl<'scope, K, V, M: crate::Mapping<K, V>> crate::Mapping<$tag_type<'scope, K>, V>
            for $map_type<'scope, K, V, M>
        {
            fn map<VV>(
                self,
                f: impl FnMut(V) -> VV,
            ) -> impl crate::Mapping<$tag_type<'scope, K>, VV> {
                $map_type {
                    _scope: self._scope,
                    _key: self._key,
                    _value: core::marker::PhantomData,
                    inner: self.inner.map(f),
                }
            }

            fn iter<'a>(&'a self) -> impl Iterator<Item = &'a V>
            where
                V: 'a,
            {
                self.inner.iter()
            }

            fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V>
            where
                V: 'a,
            {
                self.inner.iter_mut()
            }

            unsafe fn get_unchecked(&self, $tag_type(_, key): $tag_type<'scope, K>) -> &V {
                self.inner.get_unchecked(key)
            }

            unsafe fn get_unchecked_mut(
                &mut self,
                $tag_type(_, key): $tag_type<'scope, K>,
            ) -> &mut V {
                self.inner.get_unchecked_mut(key)
            }
        }
    };
}

impl_context_map!(ContextNodeMap, NodeTag);
impl_context_map!(ContextEdgeMap, EdgeTag);

/// A lifetime-parameterized wrapper around node indices.
///
/// `NodeTag` ensures that node references cannot escape the scope they were created in
/// and cannot be used with different graphs. This provides compile-time safety guarantees
/// that prevent common graph programming errors.
///
/// # Type Parameters
///
/// - `'scope`: The lifetime of the scope this tag is valid in
/// - `I`: The underlying index type
///
/// # Safety Guarantees
///
/// - **Scope Safety**: Tags cannot escape the scope they were created in
/// - **Graph Safety**: Tags from one graph cannot be used with another graph
/// - **Lifetime Safety**: Prevents use-after-remove scenarios
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<i32, ()> = VecGraph::default();
/// graph.scope_mut(|mut ctx| {
///     let node_tag = ctx.add_node(42);
///     // Use node_tag within the same scope
///     println!("Node value: {}", ctx.node(node_tag));
/// });
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct NodeTag<'scope, I>(pub(crate) crate::Invariant<'scope>, pub I);

impl<'scope, I> NodeTag<'scope, I> {
    /// Extracts the underlying index from this tag.
    ///
    /// This method allows access to the raw index when needed for interoperability
    /// with external APIs or for implementing custom graph algorithms.
    ///
    /// # Returns
    ///
    /// The underlying index value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gotgraph::prelude::*;
    ///
    /// let mut graph: VecGraph<i32, ()> = VecGraph::default();
    /// graph.scope_mut(|mut ctx| {
    ///     let node_tag = ctx.add_node(42);
    ///     let raw_index = node_tag.inner();
    ///     // raw_index now has the underlying NodeIx type
    /// });
    /// ```
    pub fn inner(self) -> I {
        self.1
    }
}

/// A lifetime-parameterized wrapper around edge indices.
///
/// `EdgeTag` ensures that edge references cannot escape the scope they were created in
/// and cannot be used with different graphs. This provides compile-time safety guarantees
/// that prevent common graph programming errors.
///
/// # Type Parameters
///
/// - `'scope`: The lifetime of the scope this tag is valid in
/// - `I`: The underlying index type
///
/// # Safety Guarantees
///
/// - **Scope Safety**: Tags cannot escape the scope they were created in
/// - **Graph Safety**: Tags from one graph cannot be used with another graph
/// - **Lifetime Safety**: Prevents use-after-remove scenarios
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct EdgeTag<'scope, I>(pub(crate) crate::Invariant<'scope>, pub I);

impl<'scope, I> EdgeTag<'scope, I> {
    /// Extracts the underlying index from this tag.
    ///
    /// This method allows access to the raw index when needed for interoperability
    /// with external APIs or for implementing custom graph algorithms.
    ///
    /// # Returns
    ///
    /// The underlying index value.
    pub fn inner(self) -> I {
        self.1
    }
}

/// A scoped context for safe graph operations.
///
/// `Context` provides a safe interface for graph operations within a specific lifetime scope.
/// All node and edge references obtained through a context are tagged with the context's
/// lifetime, preventing them from being used incorrectly.
///
/// # Type Parameters
///
/// - `'scope`: The lifetime of this context
/// - `G`: The graph type being accessed
///
/// # Key Features
///
/// - **Lifetime Safety**: All operations are bound to the context's lifetime
/// - **Type Safety**: Node and edge tags prevent cross-graph contamination
/// - **Scoped Mappings**: Create mappings that cannot escape the scope
/// - **Safe Mutations**: Mutable operations when the context has exclusive access
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<i32, &str> = VecGraph::default();
///
/// // Read-only context
/// graph.scope(|ctx| {
///     // Can read graph data but not modify it
///     for node_tag in ctx.node_indices() {
///         println!("Node: {}", ctx.node(node_tag));
///     }
/// });
///
/// // Mutable context
/// graph.scope_mut(|mut ctx| {
///     let n1 = ctx.add_node(42);
///     let n2 = ctx.add_node(100);
///     ctx.add_edge("connection", n1, n2);
/// });
/// ```
#[derive(Debug)]
#[repr(transparent)]
pub struct Context<'scope, G> {
    pub(crate) graph: G,
    pub(crate) _scope: crate::Invariant<'scope>,
}

impl<'scope, G: Graph> Graph for Context<'scope, G> {
    type Node = G::Node;
    type Edge = G::Edge;
    type NodeIx = NodeTag<'scope, G::NodeIx>;
    type EdgeIx = EdgeTag<'scope, G::EdgeIx>;

    fn exists_node_index(&self, NodeTag(_, _ix): Self::NodeIx) -> bool {
        true
    }

    fn exists_edge_index(&self, EdgeTag(_, _ix): Self::EdgeIx) -> bool {
        true
    }

    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIx> {
        self.graph.node_indices().map(|ix| NodeTag(PhantomData, ix))
    }

    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIx> {
        self.graph.edge_indices().map(|ix| EdgeTag(PhantomData, ix))
    }

    unsafe fn outgoing_edge_indices_unchecked(
        &self,
        NodeTag(_, ix): Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        self.graph
            .outgoing_edge_indices_unchecked(ix)
            .map(|ix| EdgeTag(PhantomData, ix))
    }

    unsafe fn incoming_edge_indices_unchecked(
        &self,
        NodeTag(_, ix): Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        self.graph
            .incoming_edge_indices_unchecked(ix)
            .map(|ix| EdgeTag(PhantomData, ix))
    }

    unsafe fn node_unchecked(&self, NodeTag(_, ix): Self::NodeIx) -> &Self::Node {
        self.graph.node_unchecked(ix)
    }

    unsafe fn edge_unchecked(&self, EdgeTag(_, ix): Self::EdgeIx) -> &Self::Edge {
        self.graph.edge_unchecked(ix)
    }

    unsafe fn endpoints_unchecked(&self, EdgeTag(_, ix): Self::EdgeIx) -> [Self::NodeIx; 2] {
        self.graph
            .endpoints_unchecked(ix)
            .map(|ix| NodeTag(PhantomData, ix))
    }

    unsafe fn outgoing_edge_pairs_unchecked(
        &self,
        NodeTag(_, ix): Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        self.graph
            .outgoing_edge_pairs_unchecked(ix)
            .map(|(edge_ix, edge)| (EdgeTag(PhantomData, edge_ix), edge))
    }

    unsafe fn incoming_edge_pairs_unchecked(
        &self,
        NodeTag(_, ix): Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        self.graph
            .incoming_edge_pairs_unchecked(ix)
            .map(|(edge_ix, edge)| (EdgeTag(PhantomData, edge_ix), edge))
    }

    unsafe fn node_unchecked_mut(&mut self, NodeTag(_, ix): Self::NodeIx) -> &mut Self::Node {
        self.graph.node_unchecked_mut(ix)
    }

    unsafe fn edge_unchecked_mut(&mut self, EdgeTag(_, ix): Self::EdgeIx) -> &mut Self::Edge {
        self.graph.edge_unchecked_mut(ix)
    }

    unsafe fn outgoing_edge_pairs_unchecked_mut(
        &mut self,
        NodeTag(_, ix): Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        self.graph
            .outgoing_edge_pairs_unchecked_mut(ix)
            .map(|(edge_ix, edge)| (EdgeTag(PhantomData, edge_ix), edge))
    }

    unsafe fn incoming_edge_pairs_unchecked_mut(
        &mut self,
        NodeTag(_, ix): Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        self.graph
            .incoming_edge_pairs_unchecked_mut(ix)
            .map(|(edge_ix, edge)| (EdgeTag(PhantomData, edge_ix), edge))
    }

    unsafe fn connecting_edge_pairs_unchecked_mut(
        &mut self,
        NodeTag(_, ix): Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        self.graph
            .connecting_edge_pairs_unchecked_mut(ix)
            .map(|(edge_ix, edge)| (EdgeTag(PhantomData, edge_ix), edge))
    }

    fn init_node_map<V>(
        &self,
        mut f: impl FnMut(Self::NodeIx, &Self::Node) -> V,
    ) -> impl crate::Mapping<Self::NodeIx, V> {
        ContextNodeMap {
            _scope: PhantomData,
            _key: core::marker::PhantomData,
            _value: core::marker::PhantomData,
            inner: self
                .graph
                .init_node_map(move |ix, node| f(NodeTag(PhantomData, ix), node)),
        }
    }

    fn init_edge_map<V>(
        &self,
        mut f: impl FnMut(Self::EdgeIx, &Self::Edge) -> V,
    ) -> impl crate::Mapping<Self::EdgeIx, V> {
        ContextEdgeMap {
            _scope: PhantomData,
            _key: core::marker::PhantomData,
            _value: core::marker::PhantomData,
            inner: self
                .graph
                .init_edge_map(move |ix, edge| f(EdgeTag(PhantomData, ix), edge)),
        }
    }

    unsafe fn reverse_edge_unchecked(&mut self, EdgeTag(_, edge_ix): Self::EdgeIx, NodeTag(_, new_from): Self::NodeIx, NodeTag(_, new_to): Self::NodeIx)
    where
        Self: Sized,
    {
        self.graph.reverse_edge_unchecked(edge_ix, new_from, new_to)
    }
}

impl<'scope, G: GraphUpdate> GraphUpdate for Context<'scope, G> {
    fn add_node(&mut self, node: Self::Node) -> Self::NodeIx {
        NodeTag(PhantomData, self.graph.add_node(node))
    }

    unsafe fn add_edge_unchecked(
        &mut self,
        edge: Self::Edge,
        NodeTag(_, from): Self::NodeIx,
        NodeTag(_, to): Self::NodeIx,
    ) -> Self::EdgeIx {
        EdgeTag(PhantomData, self.graph.add_edge_unchecked(edge, from, to))
    }
}

impl<'scope, G: GraphRemove> Context<'scope, G> {
    pub fn remove_nodes_edges<CN, CE>(
        mut self,
        nodes: impl IntoIterator<Item = NodeTag<'scope, G::NodeIx>>,
        edges: impl IntoIterator<Item = EdgeTag<'scope, G::EdgeIx>>,
    ) -> (CN, CE)
    where
        CN: Default + Extend<G::Node>,
        CE: Default + Extend<G::Edge>,
    {
        let node_indices = nodes.into_iter().map(|NodeTag(_, ix)| ix);
        let edge_indices = edges.into_iter().map(|EdgeTag(_, ix)| ix);
        unsafe {
            self.graph
                .remove_nodes_edges_unchecked(node_indices, edge_indices)
        }
    }
}
