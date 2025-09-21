pub mod basic;

use core::marker::PhantomData;

use basic::GraphBasic;

pub trait Graph: GraphBasic {
    // scope
    fn scope<'graph, R, F: for<'scope> FnOnce(Context<'scope, 'graph, Self>) -> R>(
        &'graph self,
        f: F,
    ) -> R {
        f(Context {
            graph: self,
            _scope: PhantomData,
        })
    }

    fn scope_mut<'graph, R, F: for<'scope> FnOnce(ContextMut<'scope, 'graph, Self>) -> R>(
        &'graph mut self,
        f: F,
    ) -> R {
        f(ContextMut {
            graph: self,
            _scope: PhantomData,
        })
    }

    // iterators
    fn nodes(&self) -> impl Iterator<Item = &Self::Node> + use<'_, Self> {
        self.node_indices()
            .map(|ix| unsafe { self.get_node_unchecked(ix) })
    }

    fn edges(&self) -> impl Iterator<Item = &Self::Edge> + use<'_, Self> {
        self.edge_indices()
            .map(|ix| unsafe { self.get_edge_unchecked(ix) })
    }

    fn nodes_mut(&mut self) -> impl Iterator<Item = &mut Self::Node> + use<'_, Self> {
        struct NodesMutIter<'a, G: GraphBasic> {
            graph: &'a mut G,
            indices: std::vec::IntoIter<G::NodeIx>,
        }

        impl<'a, G: GraphBasic> Iterator for NodesMutIter<'a, G> {
            type Item = &'a mut G::Node;

            fn next(&mut self) -> Option<Self::Item> {
                self.indices.next().map(|ix| unsafe {
                    let ptr = self.graph.get_node_unchecked_mut(ix) as *mut G::Node;
                    &mut *ptr
                })
            }
        }

        let indices: Vec<_> = self.node_indices().collect();
        NodesMutIter {
            graph: self,
            indices: indices.into_iter(),
        }
    }

    fn edges_mut(&mut self) -> impl Iterator<Item = &mut Self::Edge> + use<'_, Self> {
        struct EdgesMutIter<'a, G: GraphBasic> {
            graph: &'a mut G,
            indices: std::vec::IntoIter<G::EdgeIx>,
        }

        impl<'a, G: GraphBasic> Iterator for EdgesMutIter<'a, G> {
            type Item = &'a mut G::Edge;

            fn next(&mut self) -> Option<Self::Item> {
                self.indices.next().map(|ix| unsafe {
                    let ptr = self.graph.get_edge_unchecked_mut(ix) as *mut G::Edge;
                    &mut *ptr
                })
            }
        }

        let indices: Vec<_> = self.edge_indices().collect();
        EdgesMutIter {
            graph: self,
            indices: indices.into_iter(),
        }
    }

    fn drain<CN, CE>(&mut self) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>,
    {
        let nodes: Vec<_> = self.node_indices().collect();
        let edges: Vec<_> = self.edge_indices().collect();
        unsafe { self.remove_nodes_edges_unchecked(nodes, edges) }
    }

    // info
    fn len_nodes(&self) -> usize {
        self.nodes().count()
    }
    fn len_edges(&self) -> usize {
        self.edges().count()
    }
    fn is_empty(&self) -> bool {
        self.len_nodes() == 0 && self.len_edges() == 0
    }

    // append
    fn append(&mut self, mut other: impl Graph<Node = Self::Node, Edge = Self::Edge>) {
        use std::collections::HashMap;

        // Collect all indices and their data before draining
        let edge_data: Vec<_> = other
            .edge_indices()
            .map(|edge_ix| {
                let endpoints = unsafe { other.get_endpoints_unchecked(edge_ix) };
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

    fn insert_node(&mut self, item: Self::Node) -> Self::NodeIx {
        self.add_node(item)
    }

    // remove
    fn remove_nodes_with<F: FnMut(&Self::Node) -> bool>(
        &mut self,
        mut f: F,
    ) -> impl Iterator<Item = Self::Node> + use<'_, Self, F> {
        let to_remove: Vec<_> = self
            .node_indices()
            .filter(|&ix| unsafe { f(self.get_node_unchecked(ix)) })
            .collect();

        to_remove
            .into_iter()
            .filter_map(move |ix| self.remove_node(ix))
    }

    fn remove_edges_with<F: FnMut(&Self::Edge) -> bool>(
        &mut self,
        mut f: F,
    ) -> impl Iterator<Item = Self::Edge> + use<'_, Self, F> {
        let to_remove: Vec<_> = self
            .edge_indices()
            .filter(|&ix| unsafe { f(self.get_edge_unchecked(ix)) })
            .collect();

        to_remove
            .into_iter()
            .filter_map(move |ix| self.remove_edge(ix))
    }

    fn clear_edges(&mut self) {
        let edges: Vec<_> = self.edge_indices().collect();
        for edge_ix in edges {
            self.remove_edge(edge_ix);
        }
    }

    fn clear(&mut self) {
        let _: (Vec<Self::Node>, Vec<Self::Edge>) = self.drain();
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeTag<'scope, I>(crate::Invariant<'scope>, I);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EdgeTag<'scope, I>(crate::Invariant<'scope>, I);

#[derive(Debug)]
pub struct Context<'scope, 'graph, G: GraphBasic> {
    graph: &'graph G,
    _scope: crate::Invariant<'scope>,
}

impl<'scope, 'graph, G: GraphBasic> Context<'scope, 'graph, G> {
    fn get_graph(&self) -> &'graph G {
        self.graph
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeTag<'scope, G::NodeIx>> + use<'_, 'scope, G> {
        self.get_graph()
            .node_indices()
            .map(|ix| NodeTag(PhantomData, ix.clone()))
    }

    pub fn edges(&self) -> impl Iterator<Item = EdgeTag<'scope, G::EdgeIx>> + use<'_, 'scope, G> {
        self.get_graph()
            .edge_indices()
            .map(|ix| EdgeTag(PhantomData, ix.clone()))
    }

    pub fn get_node(&self, NodeTag(_, ix): NodeTag<'scope, G::NodeIx>) -> &'graph G::Node {
        unsafe { self.get_graph().get_node_unchecked(ix) }
    }

    pub fn get_edge(&self, EdgeTag(_, ix): EdgeTag<'scope, G::EdgeIx>) -> &'graph G::Edge {
        unsafe { self.get_graph().get_edge_unchecked(ix) }
    }

    pub fn get_outgoing_edges(
        &self,
        NodeTag(_, ix): NodeTag<'scope, G::NodeIx>,
    ) -> impl Iterator<Item = EdgeTag<'scope, G::EdgeIx>> + use<'_, 'scope, G> {
        unsafe { self.get_graph().get_outgoing_edges_unchecked(ix) }
            .map(|ix| EdgeTag(PhantomData, ix))
    }

    pub fn get_incoming_edges(
        &self,
        NodeTag(_, ix): NodeTag<'scope, G::NodeIx>,
    ) -> impl Iterator<Item = EdgeTag<'scope, G::EdgeIx>> + use<'_, 'scope, G> {
        unsafe { self.get_graph().get_incoming_edges_unchecked(ix) }
            .map(|ix| EdgeTag(PhantomData, ix))
    }

    pub fn get_endpoints(
        &self,
        EdgeTag(_, ix): EdgeTag<'scope, G::EdgeIx>,
    ) -> [NodeTag<'scope, G::NodeIx>; 2] {
        unsafe { self.get_graph().get_endpoints_unchecked(ix) }.map(|ix| NodeTag(PhantomData, ix))
    }
}

#[derive(Debug)]
pub struct ContextMut<'scope, 'graph, G: GraphBasic> {
    graph: &'graph mut G,
    _scope: crate::Invariant<'scope>,
}

impl<'scope, 'graph, G: GraphBasic> ContextMut<'scope, 'graph, G> {
    fn get_graph(&self) -> &G {
        &self.graph
    }

    fn get_graph_mut(&mut self) -> &mut G {
        &mut self.graph
    }

    pub fn nodes(&self) -> impl Iterator<Item = NodeTag<'scope, G::NodeIx>> + use<'_, 'scope, G> {
        self.get_graph()
            .node_indices()
            .map(|ix| NodeTag(PhantomData, ix))
    }

    pub fn edges(&self) -> impl Iterator<Item = EdgeTag<'scope, G::EdgeIx>> + use<'_, 'scope, G> {
        self.get_graph()
            .edge_indices()
            .map(|ix| EdgeTag(PhantomData, ix))
    }

    pub fn get_node(&self, NodeTag(_, ix): NodeTag<'scope, G::NodeIx>) -> &G::Node {
        unsafe { self.get_graph().get_node_unchecked(ix) }
    }

    pub fn get_edge(&self, EdgeTag(_, ix): EdgeTag<'scope, G::EdgeIx>) -> &G::Edge {
        unsafe { self.get_graph().get_edge_unchecked(ix) }
    }

    pub fn get_outgoing_edges(
        &self,
        NodeTag(_, ix): NodeTag<'scope, G::NodeIx>,
    ) -> impl Iterator<Item = EdgeTag<'scope, G::EdgeIx>> + use<'_, 'scope, G> {
        unsafe { self.get_graph().get_outgoing_edges_unchecked(ix) }
            .map(|ix| EdgeTag(PhantomData, ix))
    }

    pub fn get_incoming_edges(
        &self,
        NodeTag(_, ix): NodeTag<'scope, G::NodeIx>,
    ) -> impl Iterator<Item = EdgeTag<'scope, G::EdgeIx>> + use<'_, 'scope, G> {
        unsafe { self.get_graph().get_incoming_edges_unchecked(ix) }
            .map(|ix| EdgeTag(PhantomData, ix))
    }

    pub fn get_endpoints(
        &self,
        EdgeTag(_, ix): EdgeTag<'scope, G::EdgeIx>,
    ) -> [NodeTag<'scope, G::NodeIx>; 2] {
        unsafe { self.get_graph().get_endpoints_unchecked(ix) }.map(|ix| NodeTag(PhantomData, ix))
    }

    pub fn get_node_mut(&mut self, NodeTag(_, ix): NodeTag<'scope, G::NodeIx>) -> &mut G::Node {
        unsafe { self.get_graph_mut().get_node_unchecked_mut(ix) }
    }

    pub fn get_edge_mut(&mut self, EdgeTag(_, ix): EdgeTag<'scope, G::EdgeIx>) -> &mut G::Edge {
        unsafe { self.get_graph_mut().get_edge_unchecked_mut(ix) }
    }

    pub fn add_node(&mut self, node: G::Node) -> NodeTag<'scope, G::NodeIx> {
        NodeTag(PhantomData, self.get_graph_mut().add_node(node))
    }

    pub fn add_edge(
        &mut self,
        edge: G::Edge,
        NodeTag(_, from_ix): NodeTag<'scope, G::NodeIx>,
        NodeTag(_, to_ix): NodeTag<'scope, G::NodeIx>,
    ) -> EdgeTag<'scope, G::EdgeIx> {
        EdgeTag(PhantomData, unsafe {
            self.get_graph_mut()
                .add_edge_unchecked(edge, from_ix, to_ix)
        })
    }

    pub fn remove_nodes_edges<CN, CE>(
        self,
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
