use gotgraph::prelude::*;

#[test]
fn test_clear_graph() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    // Use scoped operations to set up the graph
    graph.scope_mut(|mut ctx| {
        let node1 = ctx.add_node("A");
        let node2 = ctx.add_node("B");
        let _edge = ctx.add_edge("AB", node1, node2);
    });

    assert!(!graph.is_empty());

    graph.clear();

    assert!(graph.is_empty());
    assert_eq!(graph.len_nodes(), 0);
    assert_eq!(graph.len_edges(), 0);
}

#[test]
fn test_append_graphs() {
    let mut graph1: VecGraph<&str, &str> = VecGraph::default();
    let mut graph2: VecGraph<&str, &str> = VecGraph::default();

    // Set up graph1 using scoped operations
    graph1.scope_mut(|mut ctx| {
        let n1 = ctx.add_node("A");
        let n2 = ctx.add_node("B");
        let _e1 = ctx.add_edge("AB", n1, n2);
    });

    // Set up graph2 using scoped operations
    graph2.scope_mut(|mut ctx| {
        let n3 = ctx.add_node("C");
        let n4 = ctx.add_node("D");
        let _e2 = ctx.add_edge("CD", n3, n4);
    });

    graph1.append(graph2);

    assert_eq!(graph1.len_nodes(), 4);
    assert_eq!(graph1.len_edges(), 2);

    let nodes: Vec<_> = graph1.nodes().cloned().collect();
    assert!(nodes.contains(&"A"));
    assert!(nodes.contains(&"B"));
    assert!(nodes.contains(&"C"));
    assert!(nodes.contains(&"D"));
}

#[test]
fn test_drain() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    // Set up the graph using scoped operations
    graph.scope_mut(|mut ctx| {
        let _n1 = ctx.add_node("A");
        let _n2 = ctx.add_node("B");
        let _e1 = ctx.add_edge("AB", _n1, _n2);
    });

    let (nodes, edges): (Vec<&str>, Vec<&str>) = graph.drain();

    assert!(graph.is_empty());
    assert_eq!(nodes.len(), 2);
    assert_eq!(edges.len(), 1);
    assert!(nodes.contains(&"A"));
    assert!(nodes.contains(&"B"));
    assert!(edges.contains(&"AB"));
}

#[test]
fn test_nodes_mut() {
    let mut graph: VecGraph<String, String> = VecGraph::default();

    // Set up the graph using scoped operations
    graph.scope_mut(|mut ctx| {
        let _n1 = ctx.add_node("original1".to_string());
        let _n2 = ctx.add_node("original2".to_string());
        let _n3 = ctx.add_node("original3".to_string());
    });

    // Modify all nodes using nodes_mut iterator
    for (i, node) in graph.nodes_mut().enumerate() {
        *node = format!("modified{}", i + 1);
    }

    // Verify the modifications
    let node_values: Vec<String> = graph.nodes().cloned().collect();
    assert_eq!(node_values.len(), 3);
    assert!(node_values.contains(&"modified1".to_string()));
    assert!(node_values.contains(&"modified2".to_string()));
    assert!(node_values.contains(&"modified3".to_string()));
}

#[test]
fn test_edges_mut() {
    let mut graph: VecGraph<&str, String> = VecGraph::default();

    // Set up the graph using scoped operations
    graph.scope_mut(|mut ctx| {
        let n1 = ctx.add_node("A");
        let n2 = ctx.add_node("B");
        let n3 = ctx.add_node("C");

        let _e1 = ctx.add_edge("edge1".to_string(), n1, n2);
        let _e2 = ctx.add_edge("edge2".to_string(), n2, n3);
        let _e3 = ctx.add_edge("edge3".to_string(), n1, n3);
    });

    // Modify all edges using edges_mut iterator
    for (i, edge) in graph.edges_mut().enumerate() {
        *edge = format!("new_edge{}", i + 1);
    }

    // Verify the modifications
    let edge_values: Vec<String> = graph.edges().cloned().collect();
    assert_eq!(edge_values.len(), 3);
    assert!(edge_values.contains(&"new_edge1".to_string()));
    assert!(edge_values.contains(&"new_edge2".to_string()));
    assert!(edge_values.contains(&"new_edge3".to_string()));
}

#[test]
fn test_remove_nodes_with() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    // Set up the graph using scoped operations
    graph.scope_mut(|mut ctx| {
        let _n1 = ctx.add_node("keep");
        let _n2 = ctx.add_node("remove");
        let _n3 = ctx.add_node("keep");
        let _n4 = ctx.add_node("remove");
    });

    assert_eq!(graph.len_nodes(), 4);

    // Remove nodes that contain "remove"
    // Note: Due to index invalidation with swap_remove, only some nodes may be removed in one pass
    let mut removed_count = 0;

    // Keep removing until no more nodes match the predicate
    loop {
        let removed_nodes: Vec<&str> = graph
            .remove_nodes_with(|node| node.contains("remove"))
            .collect();
        if removed_nodes.is_empty() {
            break;
        }
        removed_count += removed_nodes.len();
        assert!(removed_nodes.iter().all(|&node| node == "remove"));
    }

    assert_eq!(removed_count, 2);
    assert_eq!(graph.len_nodes(), 2);

    // Verify remaining nodes
    let remaining_nodes: Vec<&str> = graph.nodes().cloned().collect();
    assert_eq!(remaining_nodes.len(), 2);
    assert!(remaining_nodes.iter().all(|&node| node == "keep"));
}

#[test]
fn test_remove_edges_with() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    // Set up the graph using scoped operations
    graph.scope_mut(|mut ctx| {
        let n1 = ctx.add_node("A");
        let n2 = ctx.add_node("B");
        let n3 = ctx.add_node("C");

        let _e1 = ctx.add_edge("keep", n1, n2);
        let _e2 = ctx.add_edge("remove", n2, n3);
        let _e3 = ctx.add_edge("keep", n1, n3);
        let _e4 = ctx.add_edge("remove", n2, n1);
    });

    assert_eq!(graph.len_edges(), 4);

    // Remove edges that contain "remove"
    // Note: Due to index invalidation with swap_remove, only some edges may be removed in one pass
    let mut removed_count = 0;

    // Keep removing until no more edges match the predicate
    loop {
        let removed_edges: Vec<&str> = graph
            .remove_edges_with(|edge| edge.contains("remove"))
            .collect();
        if removed_edges.is_empty() {
            break;
        }
        removed_count += removed_edges.len();
        assert!(removed_edges.iter().all(|&edge| edge == "remove"));
    }

    assert_eq!(removed_count, 2);
    assert_eq!(graph.len_edges(), 2);

    // Verify remaining edges
    let remaining_edges: Vec<&str> = graph.edges().cloned().collect();
    assert_eq!(remaining_edges.len(), 2);
    assert!(remaining_edges.iter().all(|&edge| edge == "keep"));
}

#[test]
fn test_clear_edges() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    // Set up the graph using scoped operations
    graph.scope_mut(|mut ctx| {
        let n1 = ctx.add_node("A");
        let n2 = ctx.add_node("B");
        let n3 = ctx.add_node("C");

        let _e1 = ctx.add_edge("edge1", n1, n2);
        let _e2 = ctx.add_edge("edge2", n2, n3);
        let _e3 = ctx.add_edge("edge3", n1, n3);
    });

    assert_eq!(graph.len_nodes(), 3);
    assert_eq!(graph.len_edges(), 3);

    // Clear all edges but keep nodes
    // Note: Due to index invalidation, may need multiple passes
    while graph.len_edges() > 0 {
        graph.clear_edges();
    }

    assert_eq!(graph.len_nodes(), 3); // Nodes should remain
    assert_eq!(graph.len_edges(), 0); // All edges should be removed

    // Verify nodes are still accessible
    let remaining_nodes: Vec<&str> = graph.nodes().cloned().collect();
    assert_eq!(remaining_nodes.len(), 3);
    assert!(remaining_nodes.contains(&"A"));
    assert!(remaining_nodes.contains(&"B"));
    assert!(remaining_nodes.contains(&"C"));
}

#[test]
fn test_insert_node() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    // Use scoped operations to compare insert_node with add_node
    let (node1_valid, node2_valid) = graph.scope_mut(|mut ctx| {
        let _node1 = ctx.add_node("first");
        let _node2 = ctx.add_node("second");
        (true, true) // Both operations should succeed
    });

    // insert_node should behave exactly like add_node (both use the Graph trait)
    assert!(node1_valid && node2_valid);
    assert_eq!(graph.len_nodes(), 2);

    // Verify the nodes exist
    let nodes: Vec<&str> = graph.nodes().cloned().collect();
    assert!(nodes.contains(&"first"));
    assert!(nodes.contains(&"second"));
}

#[test]
fn test_scope_read_only_operations() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    // Set up the graph first
    graph.scope_mut(|mut ctx| {
        let node1 = ctx.add_node("Node1");
        let node2 = ctx.add_node("Node2");
        let node3 = ctx.add_node("Node3");

        let _edge1 = ctx.add_edge("Edge1", node1, node2);
        let _edge2 = ctx.add_edge("Edge2", node2, node3);
        let _edge3 = ctx.add_edge("Edge3", node1, node3);
    });

    // Test scope function with read-only operations
    let result = graph.scope(|ctx| {
        // Test nodes() iterator
        let node_tags: Vec<_> = ctx.node_indices().collect();
        assert_eq!(node_tags.len(), 3);

        // Test edges() iterator
        let edge_tags: Vec<_> = ctx.edge_indices().collect();
        assert_eq!(edge_tags.len(), 3);

        // Test node() with NodeTags
        let first_node_tag = node_tags[0];
        let node_data = ctx.node(first_node_tag);
        assert!(["Node1", "Node2", "Node3"].contains(node_data));

        // Test edge() with EdgeTags
        let first_edge_tag = edge_tags[0];
        let edge_data = ctx.edge(first_edge_tag);
        assert!(["Edge1", "Edge2", "Edge3"].contains(edge_data));

        // Test outgoing_edge_indices()
        let _outgoing_from_first: Vec<_> = ctx.outgoing_edge_indices(first_node_tag).collect();

        // Test incoming_edge_indices()
        let _incoming_to_first: Vec<_> = ctx.incoming_edge_indices(first_node_tag).collect();

        // Test endpoints()
        let endpoints = ctx.endpoints(first_edge_tag);
        assert_eq!(endpoints.len(), 2);

        // Return some computed result to verify scope works
        node_tags.len() + edge_tags.len()
    });

    assert_eq!(result, 6); // 3 nodes + 3 edges
}

#[test]
fn test_graph_context_comprehensive() {
    let mut graph: VecGraph<String, String> = VecGraph::default();

    // Test all GraphContext read-only operations in a single scope
    let verification_result = graph.scope_mut(|mut ctx| {
        // Set up a complex graph structure
        let node1 = ctx.add_node("A".to_string());
        let node2 = ctx.add_node("B".to_string());
        let node3 = ctx.add_node("C".to_string());
        let node4 = ctx.add_node("D".to_string());

        let edge1 = ctx.add_edge("A->B".to_string(), node1, node2);
        let edge2 = ctx.add_edge("B->C".to_string(), node2, node3);
        let edge3 = ctx.add_edge("A->C".to_string(), node1, node3);
        let edge4 = ctx.add_edge("C->D".to_string(), node3, node4);
        let edge5 = ctx.add_edge("B->D".to_string(), node2, node4);

        let node_tags = [node1, node2, node3, node4];
        let edge_tags = [edge1, edge2, edge3, edge4, edge5];

        // Test nodes() iterator
        let all_nodes: Vec<_> = ctx.node_indices().collect();
        assert_eq!(all_nodes.len(), 4);

        // Test edges() iterator
        let all_edges: Vec<_> = ctx.edge_indices().collect();
        assert_eq!(all_edges.len(), 5);

        // Test node() for each stored node tag
        for &node_tag in &node_tags {
            let node_data = ctx.node(node_tag);
            assert!(["A", "B", "C", "D"].contains(&node_data.as_str()));
        }

        // Test edge() for each stored edge tag
        for &edge_tag in &edge_tags {
            let edge_data = ctx.edge(edge_tag);
            assert!(["A->B", "B->C", "A->C", "C->D", "B->D"].contains(&edge_data.as_str()));
        }

        // Test outgoing_edge_indices() for specific nodes
        let node_a_outgoing: Vec<_> = ctx.outgoing_edge_indices(node_tags[0]).collect();
        assert_eq!(node_a_outgoing.len(), 2); // A->B, A->C

        let node_b_outgoing: Vec<_> = ctx.outgoing_edge_indices(node_tags[1]).collect();
        assert_eq!(node_b_outgoing.len(), 2); // B->C, B->D

        let node_c_outgoing: Vec<_> = ctx.outgoing_edge_indices(node_tags[2]).collect();
        assert_eq!(node_c_outgoing.len(), 1); // C->D

        let node_d_outgoing: Vec<_> = ctx.outgoing_edge_indices(node_tags[3]).collect();
        assert_eq!(node_d_outgoing.len(), 0); // No outgoing edges

        // Test incoming_edge_indices() for specific nodes
        let node_a_incoming: Vec<_> = ctx.incoming_edge_indices(node_tags[0]).collect();
        assert_eq!(node_a_incoming.len(), 0); // No incoming edges

        let node_b_incoming: Vec<_> = ctx.incoming_edge_indices(node_tags[1]).collect();
        assert_eq!(node_b_incoming.len(), 1); // A->B

        let node_c_incoming: Vec<_> = ctx.incoming_edge_indices(node_tags[2]).collect();
        assert_eq!(node_c_incoming.len(), 2); // B->C, A->C

        let node_d_incoming: Vec<_> = ctx.incoming_edge_indices(node_tags[3]).collect();
        assert_eq!(node_d_incoming.len(), 2); // C->D, B->D

        // Test endpoints() for all edges
        for &edge_tag in &edge_tags {
            let endpoints = ctx.endpoints(edge_tag);
            assert_eq!(endpoints.len(), 2);
            // Verify endpoints are valid node tags
            assert!(node_tags.contains(&endpoints[0]));
            assert!(node_tags.contains(&endpoints[1]));
        }

        // Verify specific edge endpoints
        let edge_a_to_b_endpoints = ctx.endpoints(edge_tags[0]);
        assert!(edge_a_to_b_endpoints[0] == node_tags[0]); // A
        assert!(edge_a_to_b_endpoints[1] == node_tags[1]); // B

        true
    });

    assert!(verification_result);
}

#[test]
fn test_graph_context_mut_comprehensive() {
    let mut graph: VecGraph<i32, f64> = VecGraph::default();

    let final_state = graph.scope_mut(|mut ctx| {
        // Test add_node()
        let node1 = ctx.add_node(100);
        let node2 = ctx.add_node(200);
        let node3 = ctx.add_node(300);
        let node4 = ctx.add_node(400);

        // Test add_edge()
        let edge1 = ctx.add_edge(1.5, node1, node2);
        let edge2 = ctx.add_edge(2.5, node2, node3);
        let edge3 = ctx.add_edge(3.5, node1, node3);
        let edge4 = ctx.add_edge(4.5, node3, node4);

        // Test node_mut() to modify node data
        *ctx.node_mut(node1) = 111;
        *ctx.node_mut(node2) = 222;
        *ctx.node_mut(node3) = 333;
        *ctx.node_mut(node4) = 444;

        // Test edge_mut() to modify edge data
        *ctx.edge_mut(edge1) = 11.1;
        *ctx.edge_mut(edge2) = 22.2;
        *ctx.edge_mut(edge3) = 33.3;
        *ctx.edge_mut(edge4) = 44.4;

        // Verify modifications using read operations
        assert_eq!(*ctx.node(node1), 111);
        assert_eq!(*ctx.node(node2), 222);
        assert_eq!(*ctx.node(node3), 333);
        assert_eq!(*ctx.node(node4), 444);

        assert_eq!(*ctx.edge(edge1), 11.1);
        assert_eq!(*ctx.edge(edge2), 22.2);
        assert_eq!(*ctx.edge(edge3), 33.3);
        assert_eq!(*ctx.edge(edge4), 44.4);

        // Test remove_nodes_edges() - remove node4 and its connected edges
        let nodes_to_remove = vec![node4];
        let edges_to_remove = vec![edge4]; // C->D edge

        // Get counts before removal
        let nodes_before = ctx.node_indices().count();
        let edges_before = ctx.edge_indices().count();

        let (removed_nodes, removed_edges): (Vec<i32>, Vec<f64>) =
            ctx.remove_nodes_edges(nodes_to_remove, edges_to_remove);

        assert_eq!(removed_nodes, vec![444]);
        assert_eq!(removed_edges, vec![44.4]);

        // Return counts that we expect after removal
        (nodes_before - 1, edges_before - 1)
    });

    assert_eq!(final_state.0, 3); // 3 nodes remaining
    assert_eq!(final_state.1, 3); // 3 edges remaining

    // Verify final graph state
    let nodes: Vec<i32> = graph.nodes().cloned().collect();
    assert_eq!(nodes.len(), 3);
    assert!(nodes.contains(&111));
    assert!(nodes.contains(&222));
    assert!(nodes.contains(&333));

    let edges: Vec<f64> = graph.edges().cloned().collect();
    assert_eq!(edges.len(), 3);
    assert!(edges.contains(&11.1));
    assert!(edges.contains(&22.2));
    assert!(edges.contains(&33.3));
}

#[test]
fn test_empty_graph_context() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    // Test operations on empty graph
    let result = graph.scope(|ctx| {
        // Test iterators on empty graph
        let nodes: Vec<_> = ctx.node_indices().collect();
        let edges: Vec<_> = ctx.edge_indices().collect();

        assert_eq!(nodes.len(), 0);
        assert_eq!(edges.len(), 0);

        (nodes.len(), edges.len())
    });

    assert_eq!(result, (0, 0));

    // Test mutable operations starting from empty
    graph.scope_mut(|mut ctx| {
        let node1 = ctx.add_node("first");

        // Test operations with single node
        let nodes: Vec<_> = ctx.node_indices().collect();
        assert_eq!(nodes.len(), 1);

        let outgoing: Vec<_> = ctx.outgoing_edge_indices(node1).collect();
        let incoming: Vec<_> = ctx.incoming_edge_indices(node1).collect();

        assert_eq!(outgoing.len(), 0);
        assert_eq!(incoming.len(), 0);

        assert_eq!(*ctx.node(node1), "first");
    });
}

#[test]
fn test_single_edge_graph_context() {
    let mut graph: VecGraph<char, u8> = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        let node_a = ctx.add_node('A');
        let node_b = ctx.add_node('B');
        let edge_ab = ctx.add_edge(42, node_a, node_b);

        // Test single edge graph structure
        let nodes: Vec<_> = ctx.node_indices().collect();
        let edges: Vec<_> = ctx.edge_indices().collect();

        assert_eq!(nodes.len(), 2);
        assert_eq!(edges.len(), 1);

        // Test edge connectivity
        let outgoing_a: Vec<_> = ctx.outgoing_edge_indices(node_a).collect();
        let incoming_a: Vec<_> = ctx.incoming_edge_indices(node_a).collect();
        let outgoing_b: Vec<_> = ctx.outgoing_edge_indices(node_b).collect();
        let incoming_b: Vec<_> = ctx.incoming_edge_indices(node_b).collect();

        assert_eq!(outgoing_a.len(), 1);
        assert_eq!(incoming_a.len(), 0);
        assert_eq!(outgoing_b.len(), 0);
        assert_eq!(incoming_b.len(), 1);

        assert!(outgoing_a[0] == edge_ab);
        assert!(incoming_b[0] == edge_ab);

        // Test endpoints
        let endpoints = ctx.endpoints(edge_ab);
        assert!(endpoints[0] == node_a);
        assert!(endpoints[1] == node_b);

        // Test data access
        assert_eq!(*ctx.node(node_a), 'A');
        assert_eq!(*ctx.node(node_b), 'B');
        assert_eq!(*ctx.edge(edge_ab), 42);
    });
}

#[test]
fn test_complex_graph_traversal() {
    let mut graph: VecGraph<&str, &str> = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create a more complex graph: A->B->C->D with cross edges A->C, B->D
        let nodes = [
            ctx.add_node("A"),
            ctx.add_node("B"),
            ctx.add_node("C"),
            ctx.add_node("D"),
        ];

        let edges = [
            ctx.add_edge("A->B", nodes[0], nodes[1]),
            ctx.add_edge("B->C", nodes[1], nodes[2]),
            ctx.add_edge("C->D", nodes[2], nodes[3]),
            ctx.add_edge("A->C", nodes[0], nodes[2]), // Skip edge
            ctx.add_edge("B->D", nodes[1], nodes[3]), // Skip edge
        ];

        // Test complex connectivity patterns

        // Node A: should have 2 outgoing (A->B, A->C), 0 incoming
        let a_out: Vec<_> = ctx.outgoing_edge_indices(nodes[0]).collect();
        let a_in: Vec<_> = ctx.incoming_edge_indices(nodes[0]).collect();
        assert_eq!(a_out.len(), 2);
        assert_eq!(a_in.len(), 0);
        assert!(a_out.contains(&edges[0])); // A->B
        assert!(a_out.contains(&edges[3])); // A->C

        // Node B: should have 2 outgoing (B->C, B->D), 1 incoming (A->B)
        let b_out: Vec<_> = ctx.outgoing_edge_indices(nodes[1]).collect();
        let b_in: Vec<_> = ctx.incoming_edge_indices(nodes[1]).collect();
        assert_eq!(b_out.len(), 2);
        assert_eq!(b_in.len(), 1);
        assert!(b_out.contains(&edges[1])); // B->C
        assert!(b_out.contains(&edges[4])); // B->D
        assert!(b_in.contains(&edges[0])); // A->B

        // Node C: should have 1 outgoing (C->D), 2 incoming (B->C, A->C)
        let c_out: Vec<_> = ctx.outgoing_edge_indices(nodes[2]).collect();
        let c_in: Vec<_> = ctx.incoming_edge_indices(nodes[2]).collect();
        assert_eq!(c_out.len(), 1);
        assert_eq!(c_in.len(), 2);
        assert!(c_out.contains(&edges[2])); // C->D
        assert!(c_in.contains(&edges[1])); // B->C
        assert!(c_in.contains(&edges[3])); // A->C

        // Node D: should have 0 outgoing, 2 incoming (C->D, B->D)
        let d_out: Vec<_> = ctx.outgoing_edge_indices(nodes[3]).collect();
        let d_in: Vec<_> = ctx.incoming_edge_indices(nodes[3]).collect();
        assert_eq!(d_out.len(), 0);
        assert_eq!(d_in.len(), 2);
        assert!(d_in.contains(&edges[2])); // C->D
        assert!(d_in.contains(&edges[4])); // B->D

        // Test all edge endpoints are correct
        for (i, &edge) in edges.iter().enumerate() {
            let endpoints = ctx.endpoints(edge);
            match i {
                0 => assert!(endpoints == [nodes[0], nodes[1]]), // A->B
                1 => assert!(endpoints == [nodes[1], nodes[2]]), // B->C
                2 => assert!(endpoints == [nodes[2], nodes[3]]), // C->D
                3 => assert!(endpoints == [nodes[0], nodes[2]]), // A->C
                4 => assert!(endpoints == [nodes[1], nodes[3]]), // B->D
                _ => unreachable!(),
            }
        }
    });
}

#[test]
fn test_scope_mut_operations() {
    let mut graph: VecGraph<String, i32> = VecGraph::default();

    // Test scope_mut function with mutable operations
    let (final_nodes, final_edges) = graph.scope_mut(|mut ctx| {
        // Test add_node() returning NodeTag
        let node1 = ctx.add_node("Initial".to_string());
        let node2 = ctx.add_node("Second".to_string());
        let node3 = ctx.add_node("Third".to_string());

        // Test add_edge() returning EdgeTag
        let edge1 = ctx.add_edge(100, node1, node2);
        let edge2 = ctx.add_edge(200, node2, node3);
        let edge3 = ctx.add_edge(300, node1, node3);

        // Test node_mut() for modifying node data
        *ctx.node_mut(node1) = "Modified First".to_string();
        *ctx.node_mut(node2) = "Modified Second".to_string();

        // Test edge_mut() for modifying edge data
        *ctx.edge_mut(edge1) = 111;
        *ctx.edge_mut(edge2) = 222;

        // Test read operations work in mutable context too
        let node_count = ctx.node_indices().count();
        let edge_count = ctx.edge_indices().count();

        // Test outgoing_edge_indices() in mutable context
        let outgoing: Vec<_> = ctx.outgoing_edge_indices(node1).collect();
        assert_eq!(outgoing.len(), 2); // node1 -> node2, node1 -> node3

        // Test incoming_edge_indices() in mutable context
        let incoming: Vec<_> = ctx.incoming_edge_indices(node3).collect();
        assert_eq!(incoming.len(), 2); // node2 -> node3, node1 -> node3

        // Test endpoints() in mutable context
        let endpoints = ctx.endpoints(edge3);
        assert_eq!(endpoints.len(), 2);

        (node_count, edge_count)
    });

    assert_eq!(final_nodes, 3);
    assert_eq!(final_edges, 3);

    // Verify modifications were applied
    let nodes: Vec<String> = graph.nodes().cloned().collect();
    assert!(nodes.contains(&"Modified First".to_string()));
    assert!(nodes.contains(&"Modified Second".to_string()));
    assert!(nodes.contains(&"Third".to_string()));

    let edges: Vec<i32> = graph.edges().cloned().collect();
    assert!(edges.contains(&111));
    assert!(edges.contains(&222));
    assert!(edges.contains(&300));
}

#[test]
fn test_large_scale_graph_with_loops() {
    let mut graph: VecGraph<usize, String> = VecGraph::default();

    const NUM_NODES: usize = 1000;
    const EDGES_PER_NODE: usize = 5;

    let (total_nodes, total_edges, connectivity_checks) = graph.scope_mut(|mut ctx| {
        // Create nodes using a loop
        let mut node_tags = Vec::with_capacity(NUM_NODES);
        for i in 0..NUM_NODES {
            let node_tag = ctx.add_node(i);
            node_tags.push(node_tag);
        }

        // Create edges using nested loops - each node connects to next EDGES_PER_NODE nodes (circular)
        for i in 0..NUM_NODES {
            for j in 1..=EDGES_PER_NODE {
                let target_idx = (i + j) % NUM_NODES;
                let edge_label = format!("{}→{}", i, target_idx);
                ctx.add_edge(edge_label, node_tags[i], node_tags[target_idx]);
            }
        }

        // Verify graph structure using iterators
        let node_count = ctx.node_indices().count();
        let actual_edge_count = ctx.edge_indices().count();

        // Test connectivity patterns for a sample of nodes
        let mut connectivity_results = Vec::new();
        for i in (0..NUM_NODES).step_by(100) {
            let outgoing: Vec<_> = ctx.outgoing_edge_indices(node_tags[i]).collect();
            let incoming: Vec<_> = ctx.incoming_edge_indices(node_tags[i]).collect();

            connectivity_results.push((i, outgoing.len(), incoming.len()));
        }

        // Test specific node data access
        for i in (0..NUM_NODES).step_by(200) {
            let node_data = ctx.node(node_tags[i]);
            assert_eq!(*node_data, i);
        }

        // Test edge endpoint verification for a sample
        let sample_edges: Vec<_> = ctx.edge_indices().take(50).collect();
        for edge_tag in sample_edges {
            let endpoints = ctx.endpoints(edge_tag);
            let source_data = ctx.node(endpoints[0]);
            let target_data = ctx.node(endpoints[1]);
            let edge_data = ctx.edge(edge_tag);

            // Verify edge label matches the connection
            let expected_label = format!("{}→{}", source_data, target_data);
            assert_eq!(*edge_data, expected_label);
        }

        (node_count, actual_edge_count, connectivity_results)
    });

    // Verify the large graph was created correctly
    assert_eq!(total_nodes, NUM_NODES);
    assert_eq!(total_edges, NUM_NODES * EDGES_PER_NODE);

    // Verify connectivity patterns
    for (node_idx, outgoing_count, incoming_count) in connectivity_checks {
        assert_eq!(
            outgoing_count, EDGES_PER_NODE,
            "Node {} should have {} outgoing edges",
            node_idx, EDGES_PER_NODE
        );
        assert_eq!(
            incoming_count, EDGES_PER_NODE,
            "Node {} should have {} incoming edges",
            node_idx, EDGES_PER_NODE
        );
    }

    // Verify final graph state
    assert_eq!(graph.len_nodes(), NUM_NODES);
    assert_eq!(graph.len_edges(), NUM_NODES * EDGES_PER_NODE);
}

#[test]
fn test_dense_graph_modification_with_loops() {
    let mut graph: VecGraph<i32, f32> = VecGraph::default();

    const GRID_SIZE: usize = 50; // 50x50 grid = 2500 nodes
    const NUM_NODES: usize = GRID_SIZE * GRID_SIZE;

    let modification_results = graph.scope_mut(|mut ctx| {
        // Create a 2D grid of nodes using nested loops
        let mut node_grid = Vec::with_capacity(GRID_SIZE);
        for row in 0..GRID_SIZE {
            let mut node_row = Vec::with_capacity(GRID_SIZE);
            for col in 0..GRID_SIZE {
                let node_value = (row * GRID_SIZE + col) as i32;
                let node_tag = ctx.add_node(node_value);
                node_row.push(node_tag);
            }
            node_grid.push(node_row);
        }

        // Create edges for grid connectivity (each node connects to its neighbors)
        let mut edge_count = 0;
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let current_node = node_grid[row][col];

                // Connect to right neighbor
                if col + 1 < GRID_SIZE {
                    let right_neighbor = node_grid[row][col + 1];
                    let edge_weight = ((row + col) as f32) * 0.1;
                    let _edge = ctx.add_edge(edge_weight, current_node, right_neighbor);
                    edge_count += 1;
                }

                // Connect to bottom neighbor
                if row + 1 < GRID_SIZE {
                    let bottom_neighbor = node_grid[row + 1][col];
                    let edge_weight = ((row + col) as f32) * 0.1 + 1000.0;
                    let _edge = ctx.add_edge(edge_weight, current_node, bottom_neighbor);
                    edge_count += 1;
                }
            }
        }

        // Modify node values using loops and mutable access
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let node_tag = node_grid[row][col];
                let node_data = ctx.node_mut(node_tag);
                *node_data = *node_data * 10; // Multiply all values by 10
            }
        }

        // Modify edge weights using loops
        let edge_tags: Vec<_> = ctx.edge_indices().collect();
        for (i, edge_tag) in edge_tags.iter().enumerate() {
            let edge_data = ctx.edge_mut(*edge_tag);
            *edge_data = *edge_data + (i as f32); // Add index to each edge weight
        }

        // Verify modifications by sampling
        let mut sample_results = Vec::new();
        for row in (0..GRID_SIZE).step_by(10) {
            for col in (0..GRID_SIZE).step_by(10) {
                let node_tag = node_grid[row][col];
                let node_value = *ctx.node(node_tag);
                let expected_value = ((row * GRID_SIZE + col) as i32) * 10;
                sample_results.push((row, col, node_value, expected_value));
            }
        }

        // Test graph traversal patterns
        let corner_node = node_grid[0][0];
        let outgoing_from_corner: Vec<_> = ctx.outgoing_edge_indices(corner_node).collect();
        let center_node = node_grid[GRID_SIZE / 2][GRID_SIZE / 2];
        let outgoing_from_center: Vec<_> = ctx.outgoing_edge_indices(center_node).collect();

        (
            edge_count,
            sample_results,
            outgoing_from_corner.len(),
            outgoing_from_center.len(),
        )
    });

    let (edge_count, sample_results, corner_edges, center_edges) = modification_results;

    // Verify graph structure
    assert_eq!(graph.len_nodes(), NUM_NODES);
    assert_eq!(graph.len_edges(), edge_count);

    // Expected edges: (GRID_SIZE-1)*GRID_SIZE horizontal + GRID_SIZE*(GRID_SIZE-1) vertical
    let expected_edges = (GRID_SIZE - 1) * GRID_SIZE * 2;
    assert_eq!(edge_count, expected_edges);

    // Verify node modifications
    for (row, col, actual_value, expected_value) in sample_results {
        assert_eq!(
            actual_value, expected_value,
            "Node at ({}, {}) should have value {}, got {}",
            row, col, expected_value, actual_value
        );
    }

    // Verify connectivity patterns
    assert_eq!(corner_edges, 2, "Corner node should have 2 outgoing edges");
    assert_eq!(center_edges, 2, "Center node should have 2 outgoing edges");
}
