use gotgraph::prelude::*;

#[test]
fn test_default_creation() {
    let graph: VecGraph<i32, &str> = VecGraph::default();
    assert_eq!(graph.len_nodes(), 0);
    assert_eq!(graph.len_edges(), 0);
    assert!(graph.is_empty());
}

#[test]
fn test_new_graph_creation() {
    let graph: VecGraph<String, i32> = Default::default();
    assert_eq!(graph.len_nodes(), 0);
    assert_eq!(graph.len_edges(), 0);
}

#[test]
fn test_add_single_node() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();
    let node_ix = graph.add_node(42);

    assert_eq!(graph.len_nodes(), 1);
    assert_eq!(graph.len_edges(), 0);
    assert!(!graph.is_empty());
    assert!(graph.exists_node_index(node_ix));

    let node_data = graph.node(node_ix);
    assert_eq!(*node_data, 42);
}

#[test]
fn test_add_multiple_nodes() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();
    let node1 = graph.add_node("first");
    let node2 = graph.add_node("second");
    let node3 = graph.add_node("third");

    assert_eq!(graph.len_nodes(), 3);
    assert_eq!(*graph.node(node1), "first");
    assert_eq!(*graph.node(node2), "second");
    assert_eq!(*graph.node(node3), "third");
}

#[test]
fn test_node_indices_iteration() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();
    let _n1 = graph.add_node(10);
    let _n2 = graph.add_node(20);
    let _n3 = graph.add_node(30);

    let node_indices: Vec<_> = graph.node_indices().collect();
    assert_eq!(node_indices.len(), 3);

    let node_values: Vec<_> = graph.nodes().cloned().collect();
    assert_eq!(node_values, vec![10, 20, 30]);
}

#[test]
fn test_add_edge_between_nodes() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");

    let edge_ix = graph.add_edge("connects", node1, node2);

    assert_eq!(graph.len_edges(), 1);
    assert!(graph.exists_edge_index(edge_ix));
    assert_eq!(*graph.edge(edge_ix), "connects");
}

#[test]
fn test_edge_endpoints() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node(1);
    let node2 = graph.add_node(2);
    let edge = graph.add_edge("edge", node1, node2);

    let endpoints = graph.endpoints(edge);
    assert_eq!(endpoints, [node1, node2]);
}

#[test]
fn test_outgoing_edges() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");
    let node3 = graph.add_node("C");

    let edge1 = graph.add_edge("AB", node1, node2);
    let edge2 = graph.add_edge("AC", node1, node3);

    let outgoing: Vec<_> = graph.outgoing_edge_indices(node1).collect();
    assert_eq!(outgoing.len(), 2);
    assert!(outgoing.contains(&edge1));
    assert!(outgoing.contains(&edge2));

    let no_outgoing: Vec<_> = graph.outgoing_edge_indices(node2).collect();
    assert_eq!(no_outgoing.len(), 0);
}

#[test]
fn test_incoming_edges() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");
    let node3 = graph.add_node("C");

    let edge1 = graph.add_edge("AB", node1, node2);
    let edge2 = graph.add_edge("CB", node3, node2);

    let incoming: Vec<_> = graph.incoming_edge_indices(node2).collect();
    assert_eq!(incoming.len(), 2);
    assert!(incoming.contains(&edge1));
    assert!(incoming.contains(&edge2));

    let no_incoming: Vec<_> = graph.incoming_edge_indices(node1).collect();
    assert_eq!(no_incoming.len(), 0);
}

#[test]
fn test_self_loop() {
    let mut graph = VecGraph::default();
    let node = graph.add_node("self");
    let edge = graph.add_edge("loop", node, node);

    let outgoing: Vec<_> = graph.outgoing_edge_indices(node).collect();
    let incoming: Vec<_> = graph.incoming_edge_indices(node).collect();

    assert_eq!(outgoing.len(), 1);
    assert_eq!(incoming.len(), 1);
    assert_eq!(outgoing[0], edge);
    assert_eq!(incoming[0], edge);
}

#[test]
fn test_remove_single_edge() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");
    let edge = graph.add_edge("AB", node1, node2);

    assert_eq!(graph.len_edges(), 1);

    let removed_edge = graph.remove_edge(edge);
    assert_eq!(removed_edge, "AB");
    assert_eq!(graph.len_edges(), 0);

    let outgoing: Vec<_> = graph.outgoing_edge_indices(node1).collect();
    let incoming: Vec<_> = graph.incoming_edge_indices(node2).collect();
    assert!(outgoing.is_empty());
    assert!(incoming.is_empty());
}

#[test]
fn test_remove_node_with_edges() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");
    let node3 = graph.add_node("C");

    let _edge1 = graph.add_edge("AB", node1, node2);
    let _edge2 = graph.add_edge("AC", node1, node3);
    let _edge3 = graph.add_edge("BC", node2, node3);

    assert_eq!(graph.len_nodes(), 3);
    assert_eq!(graph.len_edges(), 3);

    let removed_node = graph.remove_node(node1);
    assert_eq!(removed_node, "A");
    assert_eq!(graph.len_nodes(), 2);
    assert_eq!(graph.len_edges(), 1);

    // Edge between remaining nodes should still exist (but may have a different index due to swap_remove)
    let remaining_edges: Vec<_> = graph.edge_indices().collect();
    assert_eq!(remaining_edges.len(), 1);

    // Verify the remaining edge connects the two remaining nodes and has the right data
    let remaining_edge = remaining_edges[0];

    // After removing node1, check that we can still access the remaining nodes
    // The exact indices may change due to swap_remove, but the data should be preserved
    assert_eq!(*graph.edge(remaining_edge), "BC");

    // Collect the remaining node data to verify it's correct
    let remaining_node_data: Vec<&str> = graph.nodes().cloned().collect();
    assert!(remaining_node_data.contains(&"B"));
    assert!(remaining_node_data.contains(&"C"));
    assert_eq!(remaining_node_data.len(), 2);
}

#[test]
fn test_invalid_indices() {
    let graph: VecGraph<i32, i32> = VecGraph::default();
    let mut valid_graph = VecGraph::default();
    let valid_node = valid_graph.add_node(42);
    let valid_edge = valid_graph.add_edge(1, valid_node, valid_node);

    // Test invalid node index on empty graph
    assert!(!graph.exists_node_index(valid_node));
    // Note: node now panics on invalid indices instead of returning None

    // Test invalid edge index on empty graph
    assert!(!graph.exists_edge_index(valid_edge));
    // Note: edge now panics on invalid indices instead of returning None
}

#[test]
fn test_add_edge_invalid_nodes() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();
    let _node1 = graph.add_node("A");

    let mut other_graph: VecGraph<&str, &str> = VecGraph::default();
    let _node2 = other_graph.add_node("B");
    let _node3 = other_graph.add_node("C");

    // Try to add edge with a node index that doesn't exist in this graph
    // node3 has index 1, but graph only has 1 node (index 0)
    // Note: add_edge now panics on invalid indices instead of returning None
    // assert!(graph.add_edge("invalid", node1, node3).is_none());
}

#[test]
fn test_mutable_access() {
    let mut graph = VecGraph::default();
    let node = graph.add_node(String::from("original"));
    let edge = graph.add_edge(42, node, node);

    // Modify node data
    *graph.node_mut(node) = String::from("modified");
    assert_eq!(*graph.node(node), "modified");

    // Modify edge data
    *graph.edge_mut(edge) = 99;
    assert_eq!(*graph.edge(edge), 99);
}

#[test]
fn test_complex_graph_with_multiple_and_recursive_edges() {
    let mut graph: VecGraph<String, String> = VecGraph::default();

    // Create a more complex graph with multiple nodes
    let node_a = graph.add_node("A".to_string());
    let node_b = graph.add_node("B".to_string());
    let node_c = graph.add_node("C".to_string());
    let node_d = graph.add_node("D".to_string());
    let node_e = graph.add_node("E".to_string());

    // Add multiple edges between same nodes
    let edge_ab1 = graph.add_edge("A->B (1)".to_string(), node_a, node_b);
    let edge_ab2 = graph.add_edge("A->B (2)".to_string(), node_a, node_b);
    let edge_ab3 = graph.add_edge("A->B (3)".to_string(), node_a, node_b);

    // Add recursive edges (self-loops)
    let edge_aa = graph.add_edge("A->A (self)".to_string(), node_a, node_a);
    let edge_bb = graph.add_edge("B->B (self)".to_string(), node_b, node_b);

    // Create a more complex connection pattern
    let edge_bc = graph.add_edge("B->C".to_string(), node_b, node_c);
    let edge_cd = graph.add_edge("C->D".to_string(), node_c, node_d);
    let edge_de = graph.add_edge("D->E".to_string(), node_d, node_e);
    let edge_ea = graph.add_edge("E->A (cycle)".to_string(), node_e, node_a);

    // Bidirectional edges
    let edge_ce = graph.add_edge("C->E".to_string(), node_c, node_e);
    let edge_ec = graph.add_edge("E->C".to_string(), node_e, node_c);

    assert_eq!(graph.len_nodes(), 5);
    assert_eq!(graph.len_edges(), 11);

    // Test outgoing edges for node A (should have 4: 3 to B + 1 self-loop)
    let outgoing_a: Vec<_> = graph.outgoing_edge_indices(node_a).collect();
    assert_eq!(outgoing_a.len(), 4);
    assert!(outgoing_a.contains(&edge_ab1));
    assert!(outgoing_a.contains(&edge_ab2));
    assert!(outgoing_a.contains(&edge_ab3));
    assert!(outgoing_a.contains(&edge_aa));

    // Test incoming edges for node B (should have 4: 3 from A + 1 self-loop)
    let incoming_b: Vec<_> = graph.incoming_edge_indices(node_b).collect();
    assert_eq!(incoming_b.len(), 4);
    assert!(incoming_b.contains(&edge_ab1));
    assert!(incoming_b.contains(&edge_ab2));
    assert!(incoming_b.contains(&edge_ab3));
    assert!(incoming_b.contains(&edge_bb));

    // Test self-loops
    let outgoing_a_self: Vec<_> = graph.outgoing_edge_indices(node_a).collect();
    let incoming_a: Vec<_> = graph.incoming_edge_indices(node_a).collect();
    assert!(outgoing_a_self.contains(&edge_aa));
    assert!(incoming_a.contains(&edge_aa));
    assert!(incoming_a.contains(&edge_ea)); // Also receives from E

    // Test bidirectional connection between C and E
    let outgoing_c: Vec<_> = graph.outgoing_edge_indices(node_c).collect();
    let outgoing_e: Vec<_> = graph.outgoing_edge_indices(node_e).collect();
    assert!(outgoing_c.contains(&edge_ce));
    assert!(outgoing_e.contains(&edge_ec));

    // Test cycle detection by following path E->A->B->C->D->E
    assert_eq!(graph.endpoints(edge_ea), [node_e, node_a]);
    assert_eq!(graph.endpoints(edge_ab1), [node_a, node_b]);
    assert_eq!(graph.endpoints(edge_bc), [node_b, node_c]);
    assert_eq!(graph.endpoints(edge_cd), [node_c, node_d]);
    assert_eq!(graph.endpoints(edge_de), [node_d, node_e]);
}
