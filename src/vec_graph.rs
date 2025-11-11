use crate::graph::{Graph, GraphRemove, GraphRemoveEdge, GraphUpdate};
use crate::Mapping;
/// Node index type for `VecGraph`.
///
/// This is a newtype wrapper around `u32` that provides type safety
/// by preventing confusion between node and edge indices.
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// let mut graph: VecGraph<i32, ()> = VecGraph::default();
/// graph.scope_mut(|mut ctx| {
///     let node_idx = ctx.add_node(42);
///     // node_idx has type NodeTag<'_, NodeIx>
///     // node_idx.inner() has type NodeIx
/// });
/// ```
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeIx(u32);

/// Edge index type for `VecGraph`.
///
/// This is a newtype wrapper around `u32` that provides type safety
/// by preventing confusion between node and edge indices.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct EdgeIx(u32);

impl NodeIx {
    fn end() -> Self {
        NodeIx(u32::MAX)
    }

    fn is_end(self) -> bool {
        self.0 as i32 as i64 as u64 == u64::MAX
    }
}

impl EdgeIx {
    fn end() -> Self {
        EdgeIx(u32::MAX)
    }

    fn is_end(self) -> bool {
        self.0 as i32 as i64 as u64 == u64::MAX
    }
}

#[derive(Clone, Debug)]
struct NodeRepr<N> {
    data: N,
    // next outgoing / incoming edge
    next: [EdgeIx; 2],
}

#[derive(Clone, Debug)]
struct EdgeRepr<E> {
    data: E,
    // next outgoing / incoming edge
    next: [EdgeIx; 2],
    // start and end node
    node: [NodeIx; 2],
}

/// A vector-based graph implementation.
///
/// `VecGraph` stores nodes and edges in `Vec` containers, making it efficient
/// for dense graphs and applications that frequently add or remove elements.
/// It implements all the graph traits and supports the full scoped API.
///
/// # Type Parameters
///
/// - `N`: The type of data stored in nodes
/// - `E`: The type of data stored in edges
///
/// # Memory Layout
///
/// Internally, `VecGraph` uses linked lists embedded within vectors to maintain
/// efficient adjacency information. Each node maintains pointers to its first
/// outgoing and incoming edges, and edges maintain pointers to the next edge
/// in the chain.
///
/// # Performance Characteristics
///
/// - **Node/Edge Addition**: O(1) amortized
/// - **Node/Edge Removal**: O(degree) where degree is the number of edges connected to the node
/// - **Edge Traversal**: O(degree)
/// - **Memory Usage**: Efficient for dense graphs, some overhead for sparse graphs
///
/// # Examples
///
/// ```rust
/// use gotgraph::prelude::*;
///
/// // Create a new empty graph
/// let mut graph: VecGraph<&str, i32> = VecGraph::default();
///
/// // Add nodes and edges
/// graph.scope_mut(|mut ctx| {
///     let alice = ctx.add_node("Alice");
///     let bob = ctx.add_node("Bob");
///     let friendship = ctx.add_edge(10, alice, bob); // strength = 10
///     
///     // Query the graph within the same scope
///     println!("Alice: {}", ctx.node(alice));
///     println!("Bob: {}", ctx.node(bob));
///     println!("Friendship strength: {}", ctx.edge(friendship));
/// });
/// ```
#[derive(Clone, Debug)]
pub struct VecGraph<N, E> {
    nodes: Vec<NodeRepr<N>>,
    edges: Vec<EdgeRepr<E>>,
}

impl<N, E> Default for VecGraph<N, E> {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl<N, E> crate::graph::Graph for VecGraph<N, E> {
    type NodeIx = NodeIx;
    type EdgeIx = EdgeIx;
    type Node = N;
    type Edge = E;

    fn exists_node_index(&self, NodeIx(ix): Self::NodeIx) -> bool {
        (ix as usize) < self.nodes.len()
    }

    fn exists_edge_index(&self, EdgeIx(ix): Self::EdgeIx) -> bool {
        (ix as usize) < self.edges.len()
    }

    unsafe fn node_unchecked(&self, NodeIx(ix): Self::NodeIx) -> &Self::Node {
        debug_assert!((ix as usize) < self.nodes.len());
        &self.nodes.get_unchecked(ix as usize).data
    }

    unsafe fn edge_unchecked(&self, EdgeIx(ix): Self::EdgeIx) -> &Self::Edge {
        debug_assert!((ix as usize) < self.edges.len());
        &self.edges.get_unchecked(ix as usize).data
    }

    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIx> {
        (0..self.nodes.len()).map(|i| NodeIx(i as u32))
    }

    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIx> {
        (0..self.edges.len()).map(|i| EdgeIx(i as u32))
    }

    unsafe fn outgoing_edge_indices_unchecked(
        &self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        impl_get_edges::<false, N, E>(self, node)
    }

    unsafe fn incoming_edge_indices_unchecked(
        &self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        impl_get_edges::<true, N, E>(self, node)
    }

    unsafe fn endpoints_unchecked(&self, EdgeIx(edge): Self::EdgeIx) -> [Self::NodeIx; 2] {
        debug_assert!((edge as usize) < self.edges.len());
        let edge_repr = self.edges.get_unchecked(edge as usize);
        edge_repr.node
    }

    unsafe fn outgoing_edge_pairs_unchecked(
        &self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        impl_get_edges::<false, N, E>(self, node)
            .map(move |edge_ix| (edge_ix, unsafe { self.edge_unchecked(edge_ix) }))
    }

    unsafe fn incoming_edge_pairs_unchecked(
        &self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &Self::Edge)> {
        impl_get_edges::<true, N, E>(self, node)
            .map(move |edge_ix| (edge_ix, unsafe { self.edge_unchecked(edge_ix) }))
    }

    unsafe fn node_unchecked_mut(&mut self, NodeIx(ix): Self::NodeIx) -> &mut Self::Node {
        debug_assert!((ix as usize) < self.nodes.len());
        &mut self.nodes.get_unchecked_mut(ix as usize).data
    }

    unsafe fn edge_unchecked_mut(&mut self, EdgeIx(ix): Self::EdgeIx) -> &mut Self::Edge {
        debug_assert!((ix as usize) < self.edges.len());
        &mut self.edges.get_unchecked_mut(ix as usize).data
    }

    unsafe fn outgoing_edge_pairs_unchecked_mut(
        &mut self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        struct OutgoingEdgePairsMutIterUnchecked<'a, N, E> {
            graph: &'a mut VecGraph<N, E>,
            indices: std::vec::IntoIter<EdgeIx>,
        }

        impl<'a, N, E> Iterator for OutgoingEdgePairsMutIterUnchecked<'a, N, E> {
            type Item = (EdgeIx, &'a mut E);

            fn next(&mut self) -> Option<Self::Item> {
                self.indices.next().map(|ix| unsafe {
                    let ptr = self.graph.edge_unchecked_mut(ix) as *mut E;
                    (ix, &mut *ptr)
                })
            }
        }

        let indices: Vec<_> = unsafe { impl_get_edges::<false, N, E>(self, node) }.collect();
        OutgoingEdgePairsMutIterUnchecked {
            graph: self,
            indices: indices.into_iter(),
        }
    }

    unsafe fn incoming_edge_pairs_unchecked_mut(
        &mut self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        struct IncomingEdgePairsMutIterUnchecked<'a, N, E> {
            graph: &'a mut VecGraph<N, E>,
            indices: std::vec::IntoIter<EdgeIx>,
        }

        impl<'a, N, E> Iterator for IncomingEdgePairsMutIterUnchecked<'a, N, E> {
            type Item = (EdgeIx, &'a mut E);

            fn next(&mut self) -> Option<Self::Item> {
                self.indices.next().map(|ix| unsafe {
                    let ptr = self.graph.edge_unchecked_mut(ix) as *mut E;
                    (ix, &mut *ptr)
                })
            }
        }

        let indices: Vec<_> = unsafe { impl_get_edges::<true, N, E>(self, node) }.collect();
        IncomingEdgePairsMutIterUnchecked {
            graph: self,
            indices: indices.into_iter(),
        }
    }

    unsafe fn connecting_edge_pairs_unchecked_mut(
        &mut self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = (Self::EdgeIx, &mut Self::Edge)>
    where
        Self: Sized,
    {
        struct ConnectingEdgePairsMutIterUnchecked<'a, N, E> {
            graph: &'a mut VecGraph<N, E>,
            indices: std::vec::IntoIter<EdgeIx>,
        }

        impl<'a, N, E> Iterator for ConnectingEdgePairsMutIterUnchecked<'a, N, E> {
            type Item = (EdgeIx, &'a mut E);

            fn next(&mut self) -> Option<Self::Item> {
                self.indices.next().map(|ix| unsafe {
                    let ptr = self.graph.edge_unchecked_mut(ix) as *mut E;
                    (ix, &mut *ptr)
                })
            }
        }

        let outgoing_indices: Vec<_> =
            unsafe { impl_get_edges::<false, N, E>(self, node) }.collect();
        let incoming_indices: Vec<_> =
            unsafe { impl_get_edges::<true, N, E>(self, node) }.collect();
        let indices: Vec<_> = outgoing_indices
            .into_iter()
            .chain(incoming_indices)
            .collect();
        ConnectingEdgePairsMutIterUnchecked {
            graph: self,
            indices: indices.into_iter(),
        }
    }

    fn init_node_map<V>(
        &self,
        mut f: impl FnMut(Self::NodeIx, &Self::Node) -> V,
    ) -> impl Mapping<Self::NodeIx, V> {
        #[derive(Debug)]
        #[allow(dead_code)]
        pub struct VecNodeMap<'graph, V> {
            _graph: crate::Invariant<'graph>,
            data: Vec<V>,
        }

        impl<'graph, V> std::ops::Index<NodeIx> for VecNodeMap<'graph, V> {
            type Output = V;

            fn index(&self, NodeIx(ix): NodeIx) -> &Self::Output {
                &self.data[ix as usize]
            }
        }

        impl<'graph, V> std::ops::IndexMut<NodeIx> for VecNodeMap<'graph, V> {
            fn index_mut(&mut self, NodeIx(ix): NodeIx) -> &mut Self::Output {
                &mut self.data[ix as usize]
            }
        }

        impl<'graph, V> IntoIterator for VecNodeMap<'graph, V> {
            type Item = V;
            type IntoIter = std::vec::IntoIter<V>;

            fn into_iter(self) -> Self::IntoIter {
                self.data.into_iter()
            }
        }

        impl<'graph, V> Mapping<NodeIx, V> for VecNodeMap<'graph, V> {
            fn map<VV>(self, f: impl FnMut(V) -> VV) -> impl Mapping<NodeIx, VV> {
                VecNodeMap {
                    _graph: self._graph,
                    data: self.data.into_iter().map(f).collect(),
                }
            }

            fn iter<'a>(&'a self) -> impl Iterator<Item = &'a V>
            where
                V: 'a,
            {
                self.data.iter()
            }

            fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V>
            where
                V: 'a,
            {
                self.data.iter_mut()
            }

            unsafe fn get_unchecked(&self, NodeIx(ix): NodeIx) -> &V {
                self.data.get_unchecked(ix as usize)
            }

            unsafe fn get_unchecked_mut(&mut self, NodeIx(ix): NodeIx) -> &mut V {
                self.data.get_unchecked_mut(ix as usize)
            }
        }

        use core::marker::PhantomData;
        let data = self
            .nodes
            .iter()
            .enumerate()
            .map(|(i, node)| f(NodeIx(i as u32), &node.data))
            .collect();
        VecNodeMap {
            _graph: PhantomData,
            data,
        }
    }

    fn init_edge_map<V>(
        &self,
        mut f: impl FnMut(Self::EdgeIx, &Self::Edge) -> V,
    ) -> impl Mapping<Self::EdgeIx, V> {
        #[derive(Debug)]
        #[allow(dead_code)]
        pub struct VecEdgeMap<'graph, V> {
            _graph: crate::Invariant<'graph>,
            data: Vec<V>,
        }

        impl<'graph, V> std::ops::Index<EdgeIx> for VecEdgeMap<'graph, V> {
            type Output = V;

            fn index(&self, EdgeIx(ix): EdgeIx) -> &Self::Output {
                &self.data[ix as usize]
            }
        }

        impl<'graph, V> std::ops::IndexMut<EdgeIx> for VecEdgeMap<'graph, V> {
            fn index_mut(&mut self, EdgeIx(ix): EdgeIx) -> &mut Self::Output {
                &mut self.data[ix as usize]
            }
        }

        impl<'graph, V> IntoIterator for VecEdgeMap<'graph, V> {
            type Item = V;
            type IntoIter = std::vec::IntoIter<V>;

            fn into_iter(self) -> Self::IntoIter {
                self.data.into_iter()
            }
        }

        impl<'graph, V> Mapping<EdgeIx, V> for VecEdgeMap<'graph, V> {
            fn map<VV>(self, f: impl FnMut(V) -> VV) -> impl Mapping<EdgeIx, VV> {
                VecEdgeMap {
                    _graph: self._graph,
                    data: self.data.into_iter().map(f).collect(),
                }
            }

            fn iter<'a>(&'a self) -> impl Iterator<Item = &'a V>
            where
                V: 'a,
            {
                self.data.iter()
            }

            fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V>
            where
                V: 'a,
            {
                self.data.iter_mut()
            }

            unsafe fn get_unchecked(&self, EdgeIx(ix): EdgeIx) -> &V {
                self.data.get_unchecked(ix as usize)
            }

            unsafe fn get_unchecked_mut(&mut self, EdgeIx(ix): EdgeIx) -> &mut V {
                self.data.get_unchecked_mut(ix as usize)
            }
        }

        use core::marker::PhantomData;
        let data = self
            .edges
            .iter()
            .enumerate()
            .map(|(i, edge)| f(EdgeIx(i as u32), &edge.data))
            .collect();
        VecEdgeMap {
            _graph: PhantomData,
            data,
        }
    }

    unsafe fn reverse_edge_unchecked(&mut self, EdgeIx(edge_ix): Self::EdgeIx, new_from: Self::NodeIx, new_to: Self::NodeIx)
    where
        Self: Sized,
    {
        debug_assert!((edge_ix as usize) < self.edges.len());
        self.edges.get_unchecked_mut(edge_ix as usize).node = [new_from, new_to];
    }
}

impl<N, E> GraphUpdate for VecGraph<N, E> {
    fn add_node(&mut self, node: Self::Node) -> Self::NodeIx {
        if self.nodes.len() == u32::MAX as usize {
            panic!(
                "Cannot add more nodes: maximum capacity ({}) reached",
                u32::MAX
            );
        }
        let ix = NodeIx(self.nodes.len() as u32);
        debug_assert!(!ix.is_end());
        self.nodes.push(NodeRepr {
            data: node,
            next: [EdgeIx::end(), EdgeIx::end()],
        });
        ix
    }

    fn add_edge(&mut self, edge: Self::Edge, from: Self::NodeIx, to: Self::NodeIx) -> Self::EdgeIx {
        assert!(
            self.exists_node_index(from),
            "Node index {:?} does not exist",
            from
        );
        assert!(
            self.exists_node_index(to),
            "Node index {:?} does not exist",
            to
        );
        unsafe { self.add_edge_unchecked(edge, from, to) }
    }

    unsafe fn add_edge_unchecked(
        &mut self,
        edge: Self::Edge,
        n_from: Self::NodeIx,
        n_to: Self::NodeIx,
    ) -> Self::EdgeIx {
        if self.edges.len() == u32::MAX as usize {
            panic!(
                "Cannot add more edges: maximum capacity ({}) reached",
                u32::MAX
            );
        }
        let ix = EdgeIx(self.edges.len() as u32);
        debug_assert!(!ix.is_end());
        let next = match (n_from.0 as usize).cmp(&(n_to.0 as usize)) {
            core::cmp::Ordering::Equal => {
                debug_assert!((n_from.0 as usize) < self.nodes.len());
                let n = self.nodes.get_unchecked_mut(n_from.0 as usize);
                core::mem::replace(&mut n.next, [ix, ix])
            }
            o => {
                let (v_from, v_to) = if o == core::cmp::Ordering::Greater {
                    debug_assert!((n_from.0 as usize) < self.nodes.len());
                    debug_assert!((n_to.0 as usize) < (n_from.0 as usize));
                    let (ns1, ns2) = self.nodes.split_at_mut_unchecked(n_from.0 as usize);
                    (
                        ns2.get_unchecked_mut(0),
                        ns1.get_unchecked_mut(n_to.0 as usize),
                    )
                } else {
                    debug_assert!((n_to.0 as usize) < self.nodes.len());
                    debug_assert!((n_from.0 as usize) < (n_to.0 as usize));
                    let (ns1, ns2) = self.nodes.split_at_mut_unchecked(n_to.0 as usize);
                    (
                        ns1.get_unchecked_mut(n_from.0 as usize),
                        ns2.get_unchecked_mut(0),
                    )
                };
                [
                    core::mem::replace(&mut v_from.next[0], ix),
                    core::mem::replace(&mut v_to.next[1], ix),
                ]
            }
        };
        self.edges.push(EdgeRepr {
            data: edge,
            node: [n_from, n_to],
            next,
        });
        ix
    }
}

impl<N, E> GraphRemoveEdge for VecGraph<N, E> {
    unsafe fn remove_edge_unchecked(&mut self, EdgeIx(ix): Self::EdgeIx) -> Self::Edge {
        let ix = ix as usize;
        debug_assert!(ix < self.edges.len());
        let edge_repr = unsafe { self.edges.get_unchecked(ix) };
        let [from_node, to_node] = edge_repr.node;
        let [next_out, next_in] = edge_repr.next;

        // Remove from outgoing edge list of from_node
        debug_assert!((from_node.0 as usize) < self.nodes.len());
        if unsafe { self.nodes.get_unchecked(from_node.0 as usize).next[0] } == EdgeIx(ix as u32) {
            unsafe { self.nodes.get_unchecked_mut(from_node.0 as usize).next[0] = next_out };
        } else {
            let mut current = unsafe { self.nodes.get_unchecked(from_node.0 as usize).next[0] };
            while !current.is_end() {
                debug_assert!((current.0 as usize) < self.edges.len());
                let current_edge = unsafe { self.edges.get_unchecked_mut(current.0 as usize) };
                if current_edge.next[0] == EdgeIx(ix as u32) {
                    current_edge.next[0] = next_out;
                    break;
                }
                current = current_edge.next[0];
            }
        }

        // Remove from incoming edge list of to_node
        debug_assert!((to_node.0 as usize) < self.nodes.len());
        if unsafe { self.nodes.get_unchecked(to_node.0 as usize).next[1] } == EdgeIx(ix as u32) {
            unsafe { self.nodes.get_unchecked_mut(to_node.0 as usize).next[1] = next_in };
        } else {
            let mut current = unsafe { self.nodes.get_unchecked(to_node.0 as usize).next[1] };
            while !current.is_end() {
                debug_assert!((current.0 as usize) < self.edges.len());
                let current_edge = unsafe { self.edges.get_unchecked_mut(current.0 as usize) };
                if current_edge.next[1] == EdgeIx(ix as u32) {
                    current_edge.next[1] = next_in;
                    break;
                }
                current = current_edge.next[1];
            }
        }

        let edge_data = self.edges.swap_remove(ix).data;

        // Update edge indices after swap_remove
        if ix < self.edges.len() {
            let moved_edge_ix = EdgeIx(self.edges.len() as u32);

            // Update in node adjacency lists
            for node in &mut self.nodes {
                for next_edge in &mut node.next {
                    if *next_edge == moved_edge_ix {
                        *next_edge = EdgeIx(ix as u32);
                    }
                }
            }

            // Update in edge next pointers
            for edge in &mut self.edges {
                for next_edge in &mut edge.next {
                    if *next_edge == moved_edge_ix {
                        *next_edge = EdgeIx(ix as u32);
                    }
                }
            }
        }

        edge_data
    }
}

impl<N, E> GraphRemove for VecGraph<N, E> {
    unsafe fn remove_nodes_edges_unchecked<CN, CE>(
        &mut self,
        del_nodes: impl IntoIterator<Item = Self::NodeIx>,
        del_edges: impl IntoIterator<Item = Self::EdgeIx>,
    ) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>,
        Self: Sized,
    {
        use core::mem::MaybeUninit;
        let (mut cn, mut ce): (CN, CE) = Default::default();
        let mut del_ord_edge = (0..self.edges.len())
            .map(|i| (false, i))
            .collect::<Vec<_>>();
        let edges = core::mem::transmute::<&mut Vec<EdgeRepr<E>>, &mut Vec<MaybeUninit<EdgeRepr<E>>>>(
            &mut self.edges,
        );
        for EdgeIx(del_edge) in del_edges {
            let del_edge = del_edge as usize;
            debug_assert!(del_edge < del_ord_edge.len());
            let flag = unsafe { del_ord_edge.get_unchecked_mut(del_edge) };
            if !flag.0 {
                debug_assert!(del_edge < edges.len());
                ce.extend(core::iter::once(unsafe {
                    edges.get_unchecked(del_edge).assume_init_read().data
                }));
                flag.0 = true;
            }
        }
        let mut del_ord_node = (0..self.nodes.len())
            .map(|i| (false, i))
            .collect::<Vec<_>>();
        let nodes = core::mem::transmute::<&mut Vec<NodeRepr<N>>, &mut Vec<MaybeUninit<NodeRepr<N>>>>(
            &mut self.nodes,
        );
        for NodeIx(del_node) in del_nodes {
            let del_node = del_node as usize;
            debug_assert!(del_node < del_ord_node.len());
            let flag = unsafe { del_ord_node.get_unchecked_mut(del_node) };
            debug_assert!(del_node < nodes.len());
            let node = unsafe { nodes.get_unchecked(del_node).assume_init_read() };
            if !flag.0 {
                cn.extend(core::iter::once(node.data));
                flag.0 = true;
            }
            for EdgeIx(edge) in
                unsafe { impl_get_edges::<false, N, E>(self, NodeIx(del_node as u32)) }
                    .chain(unsafe { impl_get_edges::<true, N, E>(self, NodeIx(del_node as u32)) })
            {
                let edge = edge as usize;
                debug_assert!(edge < del_ord_edge.len());
                let flag = unsafe { del_ord_edge.get_unchecked_mut(edge) };
                if !flag.0 {
                    debug_assert!(edge < edges.len());
                    ce.extend(core::iter::once(unsafe {
                        edges.get_unchecked(edge).assume_init_read().data
                    }));
                    flag.0 = true;
                }
            }
        }
        let alive_edges = swap_remove(&mut del_ord_edge, |i, j| self.edges.swap(i, j));
        debug_assert!(alive_edges <= self.edges.len());
        unsafe { self.edges.set_len(alive_edges) };
        for edge in &mut self.edges {
            for edge_ix in &mut edge.next {
                if !(*edge_ix).is_end() {
                    debug_assert!((edge_ix.0 as usize) < del_ord_edge.len());
                    *edge_ix =
                        EdgeIx(unsafe { del_ord_edge.get_unchecked(edge_ix.0 as usize).1 as u32 });
                }
            }
        }
        for node in &mut self.nodes {
            for edge_ix in &mut node.next {
                if !(*edge_ix).is_end() {
                    debug_assert!((edge_ix.0 as usize) < del_ord_edge.len());
                    *edge_ix =
                        EdgeIx(unsafe { del_ord_edge.get_unchecked(edge_ix.0 as usize).1 as u32 });
                }
            }
        }

        let alive_nodes = swap_remove(&mut del_ord_node, |i, j| self.nodes.swap(i, j));
        unsafe { self.nodes.set_len(alive_nodes) };
        for edge in &mut self.edges {
            edge.node.iter_mut().for_each(|NodeIx(ix)| {
                debug_assert!((*ix as usize) < del_ord_node.len());
                *ix = unsafe { del_ord_node.get_unchecked(*ix as usize).1 as u32 };
            });
        }

        (cn, ce)
    }

    unsafe fn remove_node_unchecked(&mut self, node_ix: Self::NodeIx) -> Self::Node {
        // Collect all outgoing edges first
        let outgoing_edges: Vec<_> = self.outgoing_edge_indices_unchecked(node_ix).collect();
        for edge_ix in outgoing_edges {
            self.remove_edge_unchecked(edge_ix);
        }

        // Collect all incoming edges
        let incoming_edges: Vec<_> = self.incoming_edge_indices_unchecked(node_ix).collect();
        for edge_ix in incoming_edges {
            self.remove_edge_unchecked(edge_ix);
        }

        // Remove the node
        let NodeIx(ix) = node_ix;
        let ix = ix as usize;
        let node_data = self.nodes.swap_remove(ix).data;

        // Update node indices in edges after swap_remove
        if ix < self.nodes.len() {
            let moved_node_ix = NodeIx(self.nodes.len() as u32);
            for edge in &mut self.edges {
                for node_ref in &mut edge.node {
                    if *node_ref == moved_node_ix {
                        *node_ref = NodeIx(ix as u32);
                    }
                }
            }
        }

        node_data
    }
}

fn swap_remove(del_ord: &mut [(bool, usize)], mut cb: impl FnMut(usize, usize)) -> usize {
    const TO_REMOVE: bool = true;
    let mut i = 0;
    let mut j = del_ord.len() - 1;
    if del_ord.len() == 0 {
        return 0;
    }

    // SAFETY: in this loop, `0 <= i <= j < len` holds everywhere, so we have no need to check the
    // boundary.
    loop {
        // sentinel
        // SAFETY: see above
        debug_assert!(i < del_ord.len());
        let b = core::mem::replace(unsafe { &mut del_ord.get_unchecked_mut(i).0 }, !TO_REMOVE);

        while del_ord[j].0 == TO_REMOVE {
            j -= 1;
        }

        del_ord[i].0 = b;

        if i == j {
            if b == TO_REMOVE {
                return i;
            } else {
                return i + 1;
            }
        }

        // sentinel
        del_ord[j].0 = TO_REMOVE;

        // this loop ends, because the following holds:
        //   `i <= j` and `del_ord[j].0 == TO_REMOVE
        // SAFETY: see above
        while {
            debug_assert!(i < del_ord.len());
            unsafe { del_ord.get_unchecked(i).0 }
        } != TO_REMOVE
        {
            i += 1;
        }
        del_ord[j].0 = !TO_REMOVE;
        if i == j {
            return i + 1;
        }

        // tempolarily split the slice to diverge the mutable pointer.
        // it is safe, because here `i < j` holds
        // SAFETY: see above
        debug_assert!(i < j);
        debug_assert!(j < del_ord.len());
        unsafe {
            let (a_i, a_j) = del_ord.split_at_mut(j);
            debug_assert!(i < a_i.len());
            core::mem::swap(a_i.get_unchecked_mut(i), &mut a_j[0]);
        }
        cb(i, j);

        j -= 1;
    }
}

// SAFETY: the internal index of `node` is valid in `graph`
unsafe fn impl_get_edges<const IS_INCOMING: bool, N, E>(
    graph: &VecGraph<N, E>,
    NodeIx(node): NodeIx,
) -> impl Iterator<Item = EdgeIx> + use<'_, IS_INCOMING, N, E> {
    struct Iter<'a, const IS_INCOMING: bool, N, E>(&'a VecGraph<N, E>, EdgeIx);
    impl<'a, const IS_INCOMING: bool, N, E> Iterator for Iter<'a, IS_INCOMING, N, E> {
        type Item = EdgeIx;

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(next_edge_repr) = self.0.edges.get(self.1 .0 as usize) {
                let next = next_edge_repr.next[IS_INCOMING as usize];
                let next_ix = core::mem::replace(&mut self.1, next);
                Some(next_ix)
            } else {
                None
            }
        }
    }
    debug_assert!((node as usize) < graph.nodes.len());
    let node_repr = graph.nodes.get_unchecked(node as usize);
    Iter::<'_, IS_INCOMING, N, E>(graph, node_repr.next[IS_INCOMING as usize])
}
