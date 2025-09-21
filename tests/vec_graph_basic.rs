use gotgraph::{vec_graph::VecGraph, Graph, GraphBasic};

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
    assert!(graph.check_node_index(node_ix));

    let node_data = graph.get_node(node_ix).unwrap();
    assert_eq!(*node_data, 42);
}

#[test]
fn test_add_multiple_nodes() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();
    let node1 = graph.add_node("first");
    let node2 = graph.add_node("second");
    let node3 = graph.add_node("third");

    assert_eq!(graph.len_nodes(), 3);
    assert_eq!(*graph.get_node(node1).unwrap(), "first");
    assert_eq!(*graph.get_node(node2).unwrap(), "second");
    assert_eq!(*graph.get_node(node3).unwrap(), "third");
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

    let edge_ix = graph.add_edge("connects", node1, node2).unwrap();

    assert_eq!(graph.len_edges(), 1);
    assert!(graph.check_edge_index(edge_ix));
    assert_eq!(*graph.get_edge(edge_ix).unwrap(), "connects");
}

#[test]
fn test_edge_endpoints() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node(1);
    let node2 = graph.add_node(2);
    let edge = graph.add_edge("edge", node1, node2).unwrap();

    let endpoints = graph.get_endpoints(edge).unwrap();
    assert_eq!(endpoints, [node1, node2]);
}

#[test]
fn test_outgoing_edges() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");
    let node3 = graph.add_node("C");

    let edge1 = graph.add_edge("AB", node1, node2).unwrap();
    let edge2 = graph.add_edge("AC", node1, node3).unwrap();

    let outgoing: Vec<_> = graph.get_outgoing_edges(node1).unwrap().collect();
    assert_eq!(outgoing.len(), 2);
    assert!(outgoing.contains(&edge1));
    assert!(outgoing.contains(&edge2));

    let no_outgoing: Vec<_> = graph.get_outgoing_edges(node2).unwrap().collect();
    assert_eq!(no_outgoing.len(), 0);
}

#[test]
fn test_incoming_edges() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");
    let node3 = graph.add_node("C");

    let edge1 = graph.add_edge("AB", node1, node2).unwrap();
    let edge2 = graph.add_edge("CB", node3, node2).unwrap();

    let incoming: Vec<_> = graph.get_incoming_edges(node2).unwrap().collect();
    assert_eq!(incoming.len(), 2);
    assert!(incoming.contains(&edge1));
    assert!(incoming.contains(&edge2));

    let no_incoming: Vec<_> = graph.get_incoming_edges(node1).unwrap().collect();
    assert_eq!(no_incoming.len(), 0);
}

#[test]
fn test_self_loop() {
    let mut graph = VecGraph::default();
    let node = graph.add_node("self");
    let edge = graph.add_edge("loop", node, node).unwrap();

    let outgoing: Vec<_> = graph.get_outgoing_edges(node).unwrap().collect();
    let incoming: Vec<_> = graph.get_incoming_edges(node).unwrap().collect();

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
    let edge = graph.add_edge("AB", node1, node2).unwrap();

    assert_eq!(graph.len_edges(), 1);

    let removed_edge = graph.remove_edge(edge).unwrap();
    assert_eq!(removed_edge, "AB");
    assert_eq!(graph.len_edges(), 0);

    let outgoing: Vec<_> = graph.get_outgoing_edges(node1).unwrap().collect();
    let incoming: Vec<_> = graph.get_incoming_edges(node2).unwrap().collect();
    assert!(outgoing.is_empty());
    assert!(incoming.is_empty());
}

#[test]
fn test_remove_node_with_edges() {
    let mut graph = VecGraph::default();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");
    let node3 = graph.add_node("C");

    let _edge1 = graph.add_edge("AB", node1, node2).unwrap();
    let _edge2 = graph.add_edge("AC", node1, node3).unwrap();
    let _edge3 = graph.add_edge("BC", node2, node3).unwrap();

    assert_eq!(graph.len_nodes(), 3);
    assert_eq!(graph.len_edges(), 3);

    let removed_node = graph.remove_node(node1).unwrap();
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
    assert_eq!(*graph.get_edge(remaining_edge).unwrap(), "BC");
    
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
    let valid_edge = valid_graph.add_edge(1, valid_node, valid_node).unwrap();

    // Test invalid node index on empty graph
    assert!(!graph.check_node_index(valid_node));
    assert!(graph.get_node(valid_node).is_none());

    // Test invalid edge index on empty graph
    assert!(!graph.check_edge_index(valid_edge));
    assert!(graph.get_edge(valid_edge).is_none());
}

#[test]
fn test_add_edge_invalid_nodes() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();
    let node1 = graph.add_node("A");

    let mut other_graph: VecGraph<&str, &str> = VecGraph::default();
    let _node2 = other_graph.add_node("B");
    let node3 = other_graph.add_node("C");

    // Try to add edge with a node index that doesn't exist in this graph
    // node3 has index 1, but graph only has 1 node (index 0)
    assert!(graph.add_edge("invalid", node1, node3).is_none());
}

#[test]
fn test_mutable_access() {
    let mut graph = VecGraph::default();
    let node = graph.add_node(String::from("original"));
    let edge = graph.add_edge(42, node, node).unwrap();

    // Modify node data
    *graph.get_node_mut(node).unwrap() = String::from("modified");
    assert_eq!(*graph.get_node(node).unwrap(), "modified");

    // Modify edge data
    *graph.get_edge_mut(edge).unwrap() = 99;
    assert_eq!(*graph.get_edge(edge).unwrap(), 99);
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
    let edge_ab1 = graph.add_edge("A->B (1)".to_string(), node_a, node_b).unwrap();
    let edge_ab2 = graph.add_edge("A->B (2)".to_string(), node_a, node_b).unwrap();
    let edge_ab3 = graph.add_edge("A->B (3)".to_string(), node_a, node_b).unwrap();
    
    // Add recursive edges (self-loops)
    let edge_aa = graph.add_edge("A->A (self)".to_string(), node_a, node_a).unwrap();
    let edge_bb = graph.add_edge("B->B (self)".to_string(), node_b, node_b).unwrap();
    
    // Create a more complex connection pattern
    let edge_bc = graph.add_edge("B->C".to_string(), node_b, node_c).unwrap();
    let edge_cd = graph.add_edge("C->D".to_string(), node_c, node_d).unwrap();
    let edge_de = graph.add_edge("D->E".to_string(), node_d, node_e).unwrap();
    let edge_ea = graph.add_edge("E->A (cycle)".to_string(), node_e, node_a).unwrap();
    
    // Bidirectional edges
    let edge_ce = graph.add_edge("C->E".to_string(), node_c, node_e).unwrap();
    let edge_ec = graph.add_edge("E->C".to_string(), node_e, node_c).unwrap();
    
    assert_eq!(graph.len_nodes(), 5);
    assert_eq!(graph.len_edges(), 11);
    
    // Test outgoing edges for node A (should have 4: 3 to B + 1 self-loop)
    let outgoing_a: Vec<_> = graph.get_outgoing_edges(node_a).unwrap().collect();
    assert_eq!(outgoing_a.len(), 4);
    assert!(outgoing_a.contains(&edge_ab1));
    assert!(outgoing_a.contains(&edge_ab2));
    assert!(outgoing_a.contains(&edge_ab3));
    assert!(outgoing_a.contains(&edge_aa));
    
    // Test incoming edges for node B (should have 4: 3 from A + 1 self-loop)
    let incoming_b: Vec<_> = graph.get_incoming_edges(node_b).unwrap().collect();
    assert_eq!(incoming_b.len(), 4);
    assert!(incoming_b.contains(&edge_ab1));
    assert!(incoming_b.contains(&edge_ab2));
    assert!(incoming_b.contains(&edge_ab3));
    assert!(incoming_b.contains(&edge_bb));
    
    // Test self-loops
    let outgoing_a_self: Vec<_> = graph.get_outgoing_edges(node_a).unwrap().collect();
    let incoming_a: Vec<_> = graph.get_incoming_edges(node_a).unwrap().collect();
    assert!(outgoing_a_self.contains(&edge_aa));
    assert!(incoming_a.contains(&edge_aa));
    assert!(incoming_a.contains(&edge_ea)); // Also receives from E
    
    // Test bidirectional connection between C and E
    let outgoing_c: Vec<_> = graph.get_outgoing_edges(node_c).unwrap().collect();
    let outgoing_e: Vec<_> = graph.get_outgoing_edges(node_e).unwrap().collect();
    assert!(outgoing_c.contains(&edge_ce));
    assert!(outgoing_e.contains(&edge_ec));
    
    // Test cycle detection by following path E->A->B->C->D->E
    assert_eq!(graph.get_endpoints(edge_ea).unwrap(), [node_e, node_a]);
    assert_eq!(graph.get_endpoints(edge_ab1).unwrap(), [node_a, node_b]);
    assert_eq!(graph.get_endpoints(edge_bc).unwrap(), [node_b, node_c]);
    assert_eq!(graph.get_endpoints(edge_cd).unwrap(), [node_c, node_d]);
    assert_eq!(graph.get_endpoints(edge_de).unwrap(), [node_d, node_e]);
}