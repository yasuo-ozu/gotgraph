pub trait GraphBasic: Default {
    type NodeIx: 'static + Copy + Eq + core::hash::Hash + Ord;
    type EdgeIx: 'static + Copy + Eq + core::hash::Hash + Ord;
    type Node;
    type Edge;

    // Index validation
    fn check_node_index(&self, ix: Self::NodeIx) -> bool;
    fn check_edge_index(&self, ix: Self::EdgeIx) -> bool;

    // Basic unsafe access operations
    unsafe fn get_node_unchecked(&self, ix: Self::NodeIx) -> &Self::Node;
    unsafe fn get_edge_unchecked(&self, ix: Self::EdgeIx) -> &Self::Edge;
    unsafe fn get_node_unchecked_mut(&mut self, ix: Self::NodeIx) -> &mut Self::Node;
    unsafe fn get_edge_unchecked_mut(&mut self, ix: Self::EdgeIx) -> &mut Self::Edge;

    // Safe access operations
    fn get_node(&self, ix: Self::NodeIx) -> Option<&Self::Node> {
        if self.check_node_index(ix) {
            Some(unsafe { self.get_node_unchecked(ix) })
        } else {
            None
        }
    }
    fn get_edge(&self, ix: Self::EdgeIx) -> Option<&Self::Edge> {
        if self.check_edge_index(ix) {
            Some(unsafe { self.get_edge_unchecked(ix) })
        } else {
            None
        }
    }
    fn get_node_mut(&mut self, ix: Self::NodeIx) -> Option<&mut Self::Node> {
        if self.check_node_index(ix) {
            Some(unsafe { self.get_node_unchecked_mut(ix) })
        } else {
            None
        }
    }
    fn get_edge_mut(&mut self, ix: Self::EdgeIx) -> Option<&mut Self::Edge> {
        if self.check_edge_index(ix) {
            Some(unsafe { self.get_edge_unchecked_mut(ix) })
        } else {
            None
        }
    }

    // Basic structure operations
    fn node_indices(&self) -> impl Iterator<Item = Self::NodeIx>;
    fn edge_indices(&self) -> impl Iterator<Item = Self::EdgeIx>;

    // Basic graph topology operations
    unsafe fn get_outgoing_edges_unchecked(
        &self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx>;
    unsafe fn get_incoming_edges_unchecked(
        &self,
        node: Self::NodeIx,
    ) -> impl Iterator<Item = Self::EdgeIx>;
    unsafe fn get_endpoints_unchecked(&self, edge: Self::EdgeIx) -> [Self::NodeIx; 2];

    // Safe graph topology operations
    fn get_outgoing_edges(&self, node: Self::NodeIx) -> Option<impl Iterator<Item = Self::EdgeIx>> {
        if self.check_node_index(node) {
            Some(unsafe { self.get_outgoing_edges_unchecked(node) })
        } else {
            None
        }
    }
    fn get_incoming_edges(&self, node: Self::NodeIx) -> Option<impl Iterator<Item = Self::EdgeIx>> {
        if self.check_node_index(node) {
            Some(unsafe { self.get_incoming_edges_unchecked(node) })
        } else {
            None
        }
    }
    fn get_endpoints(&self, edge: Self::EdgeIx) -> Option<[Self::NodeIx; 2]> {
        if self.check_edge_index(edge) {
            Some(unsafe { self.get_endpoints_unchecked(edge) })
        } else {
            None
        }
    }

    // Mutation operations
    fn add_node(&mut self, node: Self::Node) -> Self::NodeIx;
    unsafe fn add_edge_unchecked(
        &mut self,
        edge: Self::Edge,
        from: Self::NodeIx,
        to: Self::NodeIx,
    ) -> Self::EdgeIx;

    // Safe edge addition
    fn add_edge(
        &mut self,
        edge: Self::Edge,
        from: Self::NodeIx,
        to: Self::NodeIx,
    ) -> Option<Self::EdgeIx> {
        if self.check_node_index(from) && self.check_node_index(to) {
            Some(unsafe { self.add_edge_unchecked(edge, from, to) })
        } else {
            None
        }
    }

    // Basic removal operations
    unsafe fn remove_node_unchecked(&mut self, ix: Self::NodeIx) -> Self::Node;
    unsafe fn remove_edge_unchecked(&mut self, ix: Self::EdgeIx) -> Self::Edge;
    unsafe fn remove_nodes_edges_unchecked<CN, CE>(
        &mut self,
        nodes: impl IntoIterator<Item = Self::NodeIx>,
        edges: impl IntoIterator<Item = Self::EdgeIx>,
    ) -> (CN, CE)
    where
        CN: Default + Extend<Self::Node>,
        CE: Default + Extend<Self::Edge>;

    // Safe single removal operations
    fn remove_node(&mut self, ix: Self::NodeIx) -> Option<Self::Node> {
        if self.check_node_index(ix) {
            Some(unsafe { self.remove_node_unchecked(ix) })
        } else {
            None
        }
    }

    fn remove_edge(&mut self, ix: Self::EdgeIx) -> Option<Self::Edge> {
        if self.check_edge_index(ix) {
            Some(unsafe { self.remove_edge_unchecked(ix) })
        } else {
            None
        }
    }
}
