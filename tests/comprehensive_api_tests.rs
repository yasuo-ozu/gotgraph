use gotgraph::prelude::*;
use gotgraph::Mapping;

/// Test graph with multiple nodes and edges for comprehensive testing
fn create_test_graph() -> VecGraph<i32, &'static str> {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create nodes: 0, 1, 2, 3
        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);
        let n3 = ctx.add_node(3);

        // Create edges forming a connected graph
        // 0 -> 1 -> 2 -> 3
        // 0 -> 2
        // 1 -> 3
        ctx.add_edge("0->1", n0, n1);
        ctx.add_edge("1->2", n1, n2);
        ctx.add_edge("2->3", n2, n3);
        ctx.add_edge("0->2", n0, n2);
        ctx.add_edge("1->3", n1, n3);
    });

    graph
}

#[test]
fn test_node_pairs_and_edge_pairs() {
    let graph = create_test_graph();

    // Test node_pairs
    let node_pairs: Vec<_> = graph.node_pairs().collect();
    assert_eq!(node_pairs.len(), 4);

    // Check that we have all expected nodes
    let node_values: Vec<_> = node_pairs.iter().map(|(_, &node)| node).collect();
    assert!(node_values.contains(&0));
    assert!(node_values.contains(&1));
    assert!(node_values.contains(&2));
    assert!(node_values.contains(&3));

    // Test edge_pairs
    let edge_pairs: Vec<_> = graph.edge_pairs().collect();
    assert_eq!(edge_pairs.len(), 5);

    // Check that we have all expected edges
    let edge_values: Vec<_> = edge_pairs.iter().map(|(_, &edge)| edge).collect();
    assert!(edge_values.contains(&"0->1"));
    assert!(edge_values.contains(&"1->2"));
    assert!(edge_values.contains(&"2->3"));
    assert!(edge_values.contains(&"0->2"));
    assert!(edge_values.contains(&"1->3"));
}

#[test]
fn test_node_pairs_mut_and_edge_pairs_mut() {
    let mut graph = create_test_graph();

    // Test node_pairs_mut - modify all node values
    for (_, node) in graph.node_pairs_mut() {
        *node += 10;
    }

    // Verify modifications
    let node_values: Vec<_> = graph.nodes().cloned().collect();
    assert!(node_values.contains(&10));
    assert!(node_values.contains(&11));
    assert!(node_values.contains(&12));
    assert!(node_values.contains(&13));

    // Test edge_pairs_mut - modify edge labels
    for (_, edge) in graph.edge_pairs_mut() {
        *edge = "modified";
    }

    // Verify edge modifications
    let edge_values: Vec<_> = graph.edges().cloned().collect();
    assert_eq!(edge_values.len(), 5);
    assert!(edge_values.iter().all(|&e| e == "modified"));
}

#[test]
fn test_unchecked_access_methods() {
    let mut graph = create_test_graph();

    // Get some indices to test with
    let node_indices: Vec<_> = graph.node_indices().collect();
    let edge_indices: Vec<_> = graph.edge_indices().collect();

    // Test node_unchecked
    unsafe {
        for &node_ix in &node_indices {
            let node = graph.node_unchecked(node_ix);
            assert!(*node >= 0 && *node <= 3);
        }
    }

    // Test edge_unchecked
    unsafe {
        for &edge_ix in &edge_indices {
            let edge = graph.edge_unchecked(edge_ix);
            assert!(edge.contains("->"));
        }
    }

    // Test node_unchecked_mut
    unsafe {
        for &node_ix in &node_indices {
            let node = graph.node_unchecked_mut(node_ix);
            *node += 100;
        }
    }

    // Verify node modifications
    let node_values: Vec<_> = graph.nodes().cloned().collect();
    assert!(node_values.iter().all(|&n| n >= 100));

    // Test edge_unchecked_mut
    unsafe {
        for &edge_ix in &edge_indices {
            let edge = graph.edge_unchecked_mut(edge_ix);
            *edge = "unchecked_modified";
        }
    }

    // Verify edge modifications
    let edge_values: Vec<_> = graph.edges().cloned().collect();
    assert!(edge_values.iter().all(|&e| e == "unchecked_modified"));
}

#[test]
fn test_endpoints_and_endpoints_unchecked() {
    let graph = create_test_graph();

    let edge_indices: Vec<_> = graph.edge_indices().collect();

    // Test endpoints
    for &edge_ix in &edge_indices {
        let [from, to] = graph.endpoints(edge_ix);

        // Verify endpoints are valid node indices
        assert!(graph.exists_node_index(from));
        assert!(graph.exists_node_index(to));

        // Test endpoints_unchecked gives same result
        unsafe {
            let [from_unchecked, to_unchecked] = graph.endpoints_unchecked(edge_ix);
            assert_eq!(from, from_unchecked);
            assert_eq!(to, to_unchecked);
        }
    }
}

#[test]
fn test_outgoing_edge_methods() {
    let graph = create_test_graph();

    let node_indices: Vec<_> = graph.node_indices().collect();

    for &node_ix in &node_indices {
        // Test outgoing_edges
        let outgoing_edges: Vec<_> = graph.outgoing_edges(node_ix).cloned().collect();
        let outgoing_indices: Vec<_> = graph.outgoing_edge_indices(node_ix).collect();
        let outgoing_pairs: Vec<_> = graph.outgoing_edge_pairs(node_ix).collect();

        assert_eq!(outgoing_edges.len(), outgoing_indices.len());
        assert_eq!(outgoing_edges.len(), outgoing_pairs.len());

        // Test that unchecked versions give same results
        unsafe {
            let outgoing_edges_unchecked: Vec<_> =
                graph.outgoing_edges_unchecked(node_ix).cloned().collect();
            let outgoing_indices_unchecked: Vec<_> =
                graph.outgoing_edge_indices_unchecked(node_ix).collect();
            let outgoing_pairs_unchecked: Vec<_> =
                graph.outgoing_edge_pairs_unchecked(node_ix).collect();

            assert_eq!(outgoing_edges, outgoing_edges_unchecked);
            assert_eq!(outgoing_indices, outgoing_indices_unchecked);
            assert_eq!(outgoing_pairs.len(), outgoing_pairs_unchecked.len());
        }
    }
}

#[test]
fn test_outgoing_edge_methods_mut() {
    let mut graph = create_test_graph();

    let node_indices: Vec<_> = graph.node_indices().collect();
    let first_node = node_indices[0];

    // Test outgoing_edges_mut
    for edge in graph.outgoing_edges_mut(first_node) {
        *edge = "outgoing_modified";
    }

    // Verify modifications affected outgoing edges
    let outgoing_edges: Vec<_> = graph.outgoing_edges(first_node).cloned().collect();
    assert!(outgoing_edges.iter().all(|&e| e == "outgoing_modified"));

    // Test outgoing_edge_pairs_mut
    for (_, edge) in graph.outgoing_edge_pairs_mut(first_node) {
        *edge = "pairs_modified";
    }

    // Test unchecked mutable versions
    unsafe {
        for edge in graph.outgoing_edges_unchecked_mut(first_node) {
            *edge = "unchecked_modified";
        }
    }

    unsafe {
        for (_, edge) in graph.outgoing_edge_pairs_unchecked_mut(first_node) {
            *edge = "unchecked_pairs_modified";
        }
    }
}

#[test]
fn test_incoming_edge_methods() {
    let graph = create_test_graph();

    let node_indices: Vec<_> = graph.node_indices().collect();

    for &node_ix in &node_indices {
        // Test incoming_edges
        let incoming_edges: Vec<_> = graph.incoming_edges(node_ix).cloned().collect();
        let incoming_indices: Vec<_> = graph.incoming_edge_indices(node_ix).collect();
        let incoming_pairs: Vec<_> = graph.incoming_edge_pairs(node_ix).collect();

        assert_eq!(incoming_edges.len(), incoming_indices.len());
        assert_eq!(incoming_edges.len(), incoming_pairs.len());

        // Test that unchecked versions give same results
        unsafe {
            let incoming_edges_unchecked: Vec<_> =
                graph.incoming_edges_unchecked(node_ix).cloned().collect();
            let incoming_indices_unchecked: Vec<_> =
                graph.incoming_edge_indices_unchecked(node_ix).collect();
            let incoming_pairs_unchecked: Vec<_> =
                graph.incoming_edge_pairs_unchecked(node_ix).collect();

            assert_eq!(incoming_edges, incoming_edges_unchecked);
            assert_eq!(incoming_indices, incoming_indices_unchecked);
            assert_eq!(incoming_pairs.len(), incoming_pairs_unchecked.len());
        }
    }
}

#[test]
fn test_incoming_edge_methods_mut() {
    let mut graph = create_test_graph();

    let node_indices: Vec<_> = graph.node_indices().collect();
    let last_node = node_indices[node_indices.len() - 1];

    // Test incoming_edges_mut
    for edge in graph.incoming_edges_mut(last_node) {
        *edge = "incoming_modified";
    }

    // Test incoming_edge_pairs_mut
    for (_, edge) in graph.incoming_edge_pairs_mut(last_node) {
        *edge = "incoming_pairs_modified";
    }

    // Test unchecked mutable versions
    unsafe {
        for edge in graph.incoming_edges_unchecked_mut(last_node) {
            *edge = "incoming_unchecked_modified";
        }
    }

    unsafe {
        for (_, edge) in graph.incoming_edge_pairs_unchecked_mut(last_node) {
            *edge = "incoming_unchecked_pairs_modified";
        }
    }
}

#[test]
fn test_connecting_edge_methods() {
    let graph = create_test_graph();

    let node_indices: Vec<_> = graph.node_indices().collect();

    for &node_ix in &node_indices {
        // Test connecting methods (both incoming and outgoing)
        let connecting_edges: Vec<_> = graph.connecting_edges(node_ix).cloned().collect();
        let connecting_indices: Vec<_> = graph.connecting_edge_indices(node_ix).collect();
        let connecting_pairs: Vec<_> = graph.connecting_edge_pairs(node_ix).collect();

        // Connecting should include both incoming and outgoing
        let outgoing_count = graph.outgoing_edge_indices(node_ix).count();
        let incoming_count = graph.incoming_edge_indices(node_ix).count();
        let expected_connecting = outgoing_count + incoming_count;

        assert_eq!(connecting_edges.len(), expected_connecting);
        assert_eq!(connecting_indices.len(), expected_connecting);
        assert_eq!(connecting_pairs.len(), expected_connecting);

        // Test unchecked versions
        unsafe {
            let connecting_edges_unchecked: Vec<_> =
                graph.connecting_edges_unchecked(node_ix).cloned().collect();
            let connecting_indices_unchecked: Vec<_> =
                graph.connecting_edge_indices_unchecked(node_ix).collect();
            let connecting_pairs_unchecked: Vec<_> =
                graph.connecting_edge_pairs_unchecked(node_ix).collect();

            assert_eq!(connecting_edges.len(), connecting_edges_unchecked.len());
            assert_eq!(connecting_indices.len(), connecting_indices_unchecked.len());
            assert_eq!(connecting_pairs.len(), connecting_pairs_unchecked.len());
        }
    }
}

#[test]
fn test_connecting_edge_methods_mut() {
    let mut graph = create_test_graph();

    let node_indices: Vec<_> = graph.node_indices().collect();
    let middle_node = node_indices[1]; // Node that has both incoming and outgoing edges

    // Test connecting_edges_mut
    for edge in graph.connecting_edges_mut(middle_node) {
        *edge = "connecting_modified";
    }

    // Test connecting_edge_pairs_mut
    for (_, edge) in graph.connecting_edge_pairs_mut(middle_node) {
        *edge = "connecting_pairs_modified";
    }

    // Test unchecked mutable versions
    unsafe {
        for edge in graph.connecting_edges_unchecked_mut(middle_node) {
            *edge = "connecting_unchecked_modified";
        }
    }

    unsafe {
        for (_, edge) in graph.connecting_edge_pairs_unchecked_mut(middle_node) {
            *edge = "connecting_unchecked_pairs_modified";
        }
    }
}

#[test]
fn test_add_edge_unchecked() {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        let n0 = ctx.add_node(100);
        let n1 = ctx.add_node(200);

        // Test add_edge_unchecked
        unsafe {
            let edge_ix = ctx.add_edge_unchecked("unchecked_edge", n0, n1);

            // Verify the edge was added
            assert!(ctx.exists_edge_index(edge_ix));
            assert_eq!(ctx.edge(edge_ix), &"unchecked_edge");

            let [from, to] = ctx.endpoints(edge_ix);
            assert_eq!(ctx.node(from), &100);
            assert_eq!(ctx.node(to), &200);
        }
    });
}

#[test]
fn test_remove_unchecked_methods() {
    let mut graph = create_test_graph();

    let edge_indices: Vec<_> = graph.edge_indices().collect();
    let node_indices: Vec<_> = graph.node_indices().collect();

    // Test remove_edge_unchecked
    let first_edge = edge_indices[0];
    let initial_edge_count = graph.len_edges();
    unsafe {
        let removed_edge = graph.remove_edge_unchecked(first_edge);
        assert!(removed_edge.contains("->"));
        // Note: In VecGraph, edge indices remain valid after removal (edges are just unlinked)
        // So we verify by checking that the edge count changed
        assert_eq!(graph.len_edges(), initial_edge_count - 1);
    }

    // Test remove_node_unchecked (this will also remove connected edges)
    let last_node = node_indices[node_indices.len() - 1];
    let initial_node_count = graph.len_nodes();
    unsafe {
        let removed_node = graph.remove_node_unchecked(last_node);
        assert!(removed_node >= 0 && removed_node <= 3);
        // Check node count reduced
        assert_eq!(graph.len_nodes(), initial_node_count - 1);
    }

    // Test remove_nodes_edges_unchecked
    let remaining_nodes: Vec<_> = graph.node_indices().take(1).collect();
    let remaining_edges: Vec<_> = graph.edge_indices().take(1).collect();

    unsafe {
        let (removed_nodes, removed_edges): (Vec<i32>, Vec<&str>) =
            graph.remove_nodes_edges_unchecked(remaining_nodes, remaining_edges);

        assert!(!removed_nodes.is_empty());
        assert!(!removed_edges.is_empty());
    }
}

#[test]
fn test_mapping_functionality() {
    let graph = create_test_graph();

    graph.scope(|ctx| {
        // Test init_node_map
        let node_map = ctx
            .init_node_map(|_node_ix, &node_data| format!("node_{}_{}", node_data, node_data * 2));

        // Test mapping access
        for node_ix in ctx.node_indices() {
            let mapped_value = &node_map[node_ix];
            assert!(mapped_value.starts_with("node_"));
        }

        // Test init_edge_map
        let edge_map = ctx.init_edge_map(|_edge_ix, &edge_data| format!("edge_{}", edge_data));

        // Test edge mapping access
        for edge_ix in ctx.edge_indices() {
            let mapped_value = &edge_map[edge_ix];
            assert!(mapped_value.starts_with("edge_"));
        }

        // Test mapping iteration
        let node_values: Vec<_> = node_map.iter().cloned().collect();
        assert_eq!(node_values.len(), 4);

        let edge_values: Vec<_> = edge_map.iter().cloned().collect();
        assert_eq!(edge_values.len(), 5);

        // Test into_iter
        let node_values_owned: Vec<_> = node_map.into_iter().collect();
        assert_eq!(node_values_owned.len(), 4);

        let edge_values_owned: Vec<_> = edge_map.into_iter().collect();
        assert_eq!(edge_values_owned.len(), 5);
    });
}

#[test]
fn test_mapping_get_unchecked() {
    let mut graph = create_test_graph();

    graph.scope_mut(|ctx| {
        // Create mutable mappings
        let mut node_map = ctx.init_node_map(|_, &node_data| node_data * 10);
        let mut edge_map = ctx.init_edge_map(|_, &edge_data| edge_data.to_string());

        // Test get_unchecked
        for node_ix in ctx.node_indices() {
            unsafe {
                let value = node_map.get_unchecked(node_ix);
                assert!(*value >= 0);
            }
        }

        for edge_ix in ctx.edge_indices() {
            unsafe {
                let value = edge_map.get_unchecked(edge_ix);
                assert!(value.contains("->"));
            }
        }

        // Test get_unchecked_mut
        for node_ix in ctx.node_indices() {
            unsafe {
                let value = node_map.get_unchecked_mut(node_ix);
                *value += 1000;
            }
        }

        for edge_ix in ctx.edge_indices() {
            unsafe {
                let value = edge_map.get_unchecked_mut(edge_ix);
                value.push_str("_modified");
            }
        }

        // Verify modifications
        for node_ix in ctx.node_indices() {
            assert!(node_map[node_ix] >= 1000);
        }

        for edge_ix in ctx.edge_indices() {
            assert!(edge_map[edge_ix].ends_with("_modified"));
        }
    });
}

#[test]
fn test_mapping_transformations() {
    let graph = create_test_graph();

    graph.scope(|ctx| {
        let node_map = ctx.init_node_map(|_, &node_data| node_data);

        // Test map transformation
        let transformed_map = node_map.map(|value| value * 100);

        // Verify transformation
        for node_ix in ctx.node_indices() {
            let original_node = ctx.node(node_ix);
            let transformed_value = &transformed_map[node_ix];
            assert_eq!(*transformed_value, original_node * 100);
        }

        // Test mutable iteration
        let mut edge_map = ctx.init_edge_map(|_, &edge_data| edge_data.to_string());

        for value in edge_map.iter_mut() {
            value.push_str("_suffix");
        }

        // Verify mutable iteration worked
        for edge_ix in ctx.edge_indices() {
            assert!(edge_map[edge_ix].ends_with("_suffix"));
        }
    });
}

#[test]
fn test_edge_graph_comprehensive() {
    let mut graph: VecGraph<i32, String> = VecGraph::default();

    // Create a more complex graph for thorough testing
    graph.scope_mut(|mut ctx| {
        let nodes: Vec<_> = (0..5).map(|i| ctx.add_node(i * 10)).collect();

        // Create a more complex edge structure
        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let edge_label = format!("{}->{}", i, j);
                ctx.add_edge(edge_label, nodes[i], nodes[j]);
            }
        }
    });

    // Test that all methods work with a larger graph
    let node_count = graph.len_nodes();
    let edge_count = graph.len_edges();

    assert_eq!(node_count, 5);
    assert_eq!(edge_count, 10); // Complete graph on 5 nodes has 10 edges

    // Test all iterator methods return consistent counts
    assert_eq!(graph.nodes().count(), node_count);
    assert_eq!(graph.edges().count(), edge_count);
    assert_eq!(graph.node_pairs().count(), node_count);
    assert_eq!(graph.edge_pairs().count(), edge_count);

    // Test that connecting edges for each node includes all edges touching that node
    for node_ix in graph.node_indices() {
        let outgoing = graph.outgoing_edge_indices(node_ix).count();
        let incoming = graph.incoming_edge_indices(node_ix).count();
        let connecting = graph.connecting_edge_indices(node_ix).count();

        assert_eq!(connecting, outgoing + incoming);
    }
}
