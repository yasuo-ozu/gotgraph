use crate::GraphBasic;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct NodeIx(usize);
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct EdgeIx(usize);

impl NodeIx {
    fn end() -> Self {
        NodeIx(usize::MAX)
    }
}

impl EdgeIx {
    fn end() -> Self {
        EdgeIx(usize::MAX)
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

impl<N, E> GraphBasic for VecGraph<N, E> {
    type NodeIx = NodeIx;
    type EdgeIx = EdgeIx;
    type Node = N;
    type Edge = E;

    fn check_node_index(&self, NodeIx(ix): Self::NodeIx) -> bool {
        ix < self.nodes.len()
    }

    fn check_edge_index(&self, EdgeIx(ix): Self::EdgeIx) -> bool {
        ix < self.edges.len()
    }

    unsafe fn get_node_unchecked(&self, NodeIx(ix): Self::NodeIx) -> &Self::Node {
        debug_assert!(ix < self.nodes.len());
        &self.nodes.get_unchecked(ix).data
    }

    unsafe fn get_edge_unchecked(&self, EdgeIx(ix): Self::EdgeIx) -> &Self::Edge {
        debug_assert!(ix < self.edges.len());
        &self.edges.get_unchecked(ix).data
    }

    unsafe fn get_node_unchecked_mut(&mut self, NodeIx(ix): Self::NodeIx) -> &mut Self::Node {
        debug_assert!(ix < self.nodes.len());
        &mut self.nodes.get_unchecked_mut(ix).data
    }

    unsafe fn get_edge_unchecked_mut(&mut self, EdgeIx(ix): Self::EdgeIx) -> &mut Self::Edge {
        debug_assert!(ix < self.edges.len());
        &mut self.edges.get_unchecked_mut(ix).data
    }

    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIx> {
        (0..self.nodes.len()).map(NodeIx)
    }

    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIx> {
        (0..self.edges.len()).map(EdgeIx)
    }

    unsafe fn get_outgoing_edges_unchecked(
        &self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        impl_get_edges::<false, N, E>(self, node)
    }

    unsafe fn get_incoming_edges_unchecked(
        &self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx> {
        impl_get_edges::<true, N, E>(self, node)
    }

    unsafe fn get_endpoints_unchecked(&self, EdgeIx(edge): Self::EdgeIx) -> [Self::NodeIx; 2] {
        debug_assert!(edge < self.edges.len());
        let edge_repr = self.edges.get_unchecked(edge);
        edge_repr.node
    }

    fn add_node(&mut self, node: Self::Node) -> Self::NodeIx {
        let ix = NodeIx(self.nodes.len());
        debug_assert!(ix != NodeIx::end());
        self.nodes.push(NodeRepr {
            data: node,
            next: [EdgeIx::end(), EdgeIx::end()],
        });
        ix
    }

    unsafe fn add_edge_unchecked(
        &mut self,
        edge: Self::Edge,
        n_from: Self::NodeIx,
        n_to: Self::NodeIx,
    ) -> Self::EdgeIx {
        let ix = EdgeIx(self.edges.len());
        debug_assert!(ix != EdgeIx::end());
        let next = match n_from.0.cmp(&n_to.0) {
            core::cmp::Ordering::Equal => {
                debug_assert!(n_from.0 < self.nodes.len());
                let n = self.nodes.get_unchecked_mut(n_from.0);
                core::mem::replace(&mut n.next, [ix, ix])
            }
            o => {
                let (v_from, v_to) = if o == core::cmp::Ordering::Greater {
                    debug_assert!(n_from.0 < self.nodes.len());
                    debug_assert!(n_to.0 < n_from.0);
                    let (ns1, ns2) = self.nodes.split_at_mut_unchecked(n_from.0);
                    (ns2.get_unchecked_mut(0), ns1.get_unchecked_mut(n_to.0))
                } else {
                    debug_assert!(n_to.0 < self.nodes.len());
                    debug_assert!(n_from.0 < n_to.0);
                    let (ns1, ns2) = self.nodes.split_at_mut_unchecked(n_to.0);
                    (ns1.get_unchecked_mut(n_from.0), ns2.get_unchecked_mut(0))
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

    unsafe fn remove_nodes_edges_unchecked<CN, CE>(
        &mut self,
        del_nodes: impl IntoIterator<Item = Self::NodeIx>,
        del_edges: impl IntoIterator<Item = Self::EdgeIx>,
    ) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>,
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
            debug_assert!(del_edge < del_ord_edge.len());
            let flag = del_ord_edge.get_unchecked_mut(del_edge);
            if !flag.0 {
                debug_assert!(del_edge < edges.len());
                ce.extend(core::iter::once(
                    edges.get_unchecked(del_edge).assume_init_read().data,
                ));
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
            debug_assert!(del_node < del_ord_node.len());
            let flag = del_ord_node.get_unchecked_mut(del_node);
            debug_assert!(del_node < nodes.len());
            let node = nodes.get_unchecked(del_node).assume_init_read();
            if !flag.0 {
                cn.extend(core::iter::once(node.data));
                flag.0 = true;
            }
            for EdgeIx(edge) in impl_get_edges::<false, N, E>(self, NodeIx(del_node))
                .chain(impl_get_edges::<true, N, E>(self, NodeIx(del_node)))
            {
                debug_assert!(edge < del_ord_edge.len());
                let flag = del_ord_edge.get_unchecked_mut(edge);
                if !flag.0 {
                    debug_assert!(edge < edges.len());
                    edges.get_unchecked(edge).assume_init_read();
                    flag.0 = true;
                }
            }
        }
        let alive_edges = swap_remove(&mut del_ord_edge, |i, j| self.edges.swap(i, j));
        debug_assert!(alive_edges <= self.edges.len());
        self.edges.set_len(alive_edges);
        for edge in &mut self.edges {
            for edge_ix in &mut edge.next {
                if *edge_ix != EdgeIx::end() {
                    debug_assert!(edge_ix.0 < del_ord_edge.len());
                    *edge_ix = EdgeIx(del_ord_edge.get_unchecked(edge_ix.0).1);
                }
            }
        }
        for node in &mut self.nodes {
            for edge_ix in &mut node.next {
                if *edge_ix != EdgeIx::end() {
                    debug_assert!(edge_ix.0 < del_ord_edge.len());
                    *edge_ix = EdgeIx(del_ord_edge.get_unchecked(edge_ix.0).1);
                }
            }
        }

        let alive_nodes = swap_remove(&mut del_ord_node, |i, j| self.nodes.swap(i, j));
        self.nodes.set_len(alive_nodes);
        for edge in &mut self.edges {
            edge.node.iter_mut().for_each(|NodeIx(ix)| {
                debug_assert!(*ix < del_ord_node.len());
                *ix = del_ord_node.get_unchecked(*ix).1;
            });
        }

        (cn, ce)
    }

    unsafe fn remove_node_unchecked(&mut self, node_ix: Self::NodeIx) -> Self::Node {
        // Collect all outgoing edges first
        let outgoing_edges: Vec<_> = self.get_outgoing_edges_unchecked(node_ix).collect();
        for edge_ix in outgoing_edges {
            self.remove_edge_unchecked(edge_ix);
        }

        // Collect all incoming edges
        let incoming_edges: Vec<_> = self.get_incoming_edges_unchecked(node_ix).collect();
        for edge_ix in incoming_edges {
            self.remove_edge_unchecked(edge_ix);
        }

        // Remove the node
        let NodeIx(ix) = node_ix;
        let node_data = self.nodes.swap_remove(ix).data;

        // Update node indices in edges after swap_remove
        if ix < self.nodes.len() {
            let moved_node_ix = NodeIx(self.nodes.len());
            for edge in &mut self.edges {
                for node_ref in &mut edge.node {
                    if *node_ref == moved_node_ix {
                        *node_ref = NodeIx(ix);
                    }
                }
            }
        }

        node_data
    }

    unsafe fn remove_edge_unchecked(&mut self, EdgeIx(ix): Self::EdgeIx) -> Self::Edge {
        debug_assert!(ix < self.edges.len());
        let edge_repr = self.edges.get_unchecked(ix);
        let [from_node, to_node] = edge_repr.node;
        let [next_out, next_in] = edge_repr.next;

        // Remove from outgoing edge list of from_node
        debug_assert!(from_node.0 < self.nodes.len());
        if self.nodes.get_unchecked(from_node.0).next[0] == EdgeIx(ix) {
            self.nodes.get_unchecked_mut(from_node.0).next[0] = next_out;
        } else {
            let mut current = self.nodes.get_unchecked(from_node.0).next[0];
            while current != EdgeIx::end() {
                debug_assert!(current.0 < self.edges.len());
                let current_edge = self.edges.get_unchecked_mut(current.0);
                if current_edge.next[0] == EdgeIx(ix) {
                    current_edge.next[0] = next_out;
                    break;
                }
                current = current_edge.next[0];
            }
        }

        // Remove from incoming edge list of to_node
        debug_assert!(to_node.0 < self.nodes.len());
        if self.nodes.get_unchecked(to_node.0).next[1] == EdgeIx(ix) {
            self.nodes.get_unchecked_mut(to_node.0).next[1] = next_in;
        } else {
            let mut current = self.nodes.get_unchecked(to_node.0).next[1];
            while current != EdgeIx::end() {
                debug_assert!(current.0 < self.edges.len());
                let current_edge = self.edges.get_unchecked_mut(current.0);
                if current_edge.next[1] == EdgeIx(ix) {
                    current_edge.next[1] = next_in;
                    break;
                }
                current = current_edge.next[1];
            }
        }

        let edge_data = self.edges.swap_remove(ix).data;

        // Update edge indices after swap_remove
        if ix < self.edges.len() {
            let moved_edge_ix = EdgeIx(self.edges.len());

            // Update in node adjacency lists
            for node in &mut self.nodes {
                for next_edge in &mut node.next {
                    if *next_edge == moved_edge_ix {
                        *next_edge = EdgeIx(ix);
                    }
                }
            }

            // Update in edge next pointers
            for edge in &mut self.edges {
                for next_edge in &mut edge.next {
                    if *next_edge == moved_edge_ix {
                        *next_edge = EdgeIx(ix);
                    }
                }
            }
        }

        edge_data
    }
}

impl<N, E> crate::Graph for VecGraph<N, E> {}

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
            if self.1 == EdgeIx::end() {
                None
            } else {
                debug_assert!(self.1 .0 < self.0.edges.len());
                let next_edge_repr = unsafe { self.0.edges.get_unchecked(self.1 .0) };
                let next = next_edge_repr.next[IS_INCOMING as usize];
                let next_ix = core::mem::replace(&mut self.1, next);
                Some(next_ix)
            }
        }
    }
    debug_assert!(node < graph.nodes.len());
    let node_repr = graph.nodes.get_unchecked(node);
    Iter::<'_, IS_INCOMING, N, E>(graph, node_repr.next[IS_INCOMING as usize])
}
