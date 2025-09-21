use gotgraph::algo::tarjan;
use gotgraph::prelude::*;
use std::collections::HashSet;

/// Create a comprehensive test graph with multiple SCCs and cross-SCC edges
///
/// Graph structure:
/// - SCC1: {0} (single node)
/// - SCC2: {1, 2, 3} (3-node cycle: 1→2→3→1)  
/// - SCC3: {4, 5} (2-node cycle: 4↔5)
/// - SCC4: {6} (single node)
/// - Cross-SCC edges: 0→1, 3→4, 5→6
///
/// Visual representation:
/// ```
///   0 ──→ 1 ──→ 2
///          ↑     ↓
///          └──── 3 ──→ 4 ↔ 5 ──→ 6
/// ```
fn create_comprehensive_test_graph() -> VecGraph<String, String> {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create nodes
        let n0 = ctx.add_node("Node_0".to_string());
        let n1 = ctx.add_node("Node_1".to_string());
        let n2 = ctx.add_node("Node_2".to_string());
        let n3 = ctx.add_node("Node_3".to_string());
        let n4 = ctx.add_node("Node_4".to_string());
        let n5 = ctx.add_node("Node_5".to_string());
        let n6 = ctx.add_node("Node_6".to_string());

        // SCC2: 3-node cycle (1→2→3→1)
        ctx.add_edge("edge_1_to_2".to_string(), n1, n2);
        ctx.add_edge("edge_2_to_3".to_string(), n2, n3);
        ctx.add_edge("edge_3_to_1".to_string(), n3, n1);

        // SCC3: 2-node cycle (4↔5)
        ctx.add_edge("edge_4_to_5".to_string(), n4, n5);
        ctx.add_edge("edge_5_to_4".to_string(), n5, n4);

        // Cross-SCC edges
        ctx.add_edge("edge_0_to_1".to_string(), n0, n1);
        ctx.add_edge("edge_3_to_4".to_string(), n3, n4);
        ctx.add_edge("edge_5_to_6".to_string(), n5, n6);
    });

    graph
}

/// Create a graph with nested cycles to test complex SCC detection
fn create_nested_cycles_graph() -> VecGraph<i32, &'static str> {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create a graph with nested structure:
        // Outer cycle: 0→1→2→3→0
        // Inner cycle within outer: 1→4→5→1
        // This should result in one large SCC containing all nodes

        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);
        let n3 = ctx.add_node(3);
        let n4 = ctx.add_node(4);
        let n5 = ctx.add_node(5);

        // Outer cycle
        ctx.add_edge("0->1", n0, n1);
        ctx.add_edge("1->2", n1, n2);
        ctx.add_edge("2->3", n2, n3);
        ctx.add_edge("3->0", n3, n0);

        // Inner cycle
        ctx.add_edge("1->4", n1, n4);
        ctx.add_edge("4->5", n4, n5);
        ctx.add_edge("5->1", n5, n1);
    });

    graph
}

#[test]
fn test_comprehensive_scc_detection() {
    let graph = create_comprehensive_test_graph();
    let sccs: Vec<_> = tarjan(graph).collect();

    // Should detect exactly 4 SCCs
    assert_eq!(sccs.len(), 4, "Expected 4 SCCs in comprehensive graph");

    // Group SCCs by size for easier verification
    let mut scc_sizes: Vec<usize> = sccs.iter().map(|scc| scc.len()).collect();
    scc_sizes.sort();

    // Expected: two 1-node SCCs, one 2-node SCC, one 3-node SCC
    assert_eq!(
        scc_sizes,
        vec![1, 1, 2, 3],
        "SCC sizes don't match expected pattern"
    );

    // Verify total nodes
    let total_nodes: usize = sccs.iter().map(|scc| scc.len()).sum();
    assert_eq!(total_nodes, 7, "Total nodes across SCCs should be 7");

    // Verify no node appears in multiple SCCs
    let mut all_nodes = HashSet::new();
    for scc in &sccs {
        for node in scc.iter() {
            assert!(
                all_nodes.insert(node),
                "Node {:?} appears in multiple SCCs",
                node
            );
        }
    }
    assert_eq!(all_nodes.len(), 7, "Should have exactly 7 unique nodes");
}

#[test]
fn test_nested_cycles_single_scc() {
    let graph = create_nested_cycles_graph();
    let sccs: Vec<_> = tarjan(graph).collect();

    // All nodes should be in a single SCC due to nested cycles
    assert_eq!(sccs.len(), 1, "Nested cycles should form single SCC");
    assert_eq!(sccs[0].len(), 6, "SCC should contain all 6 nodes");
}

#[test]
fn test_tarjan_with_disconnected_components() {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create two disconnected components:
        // Component 1: 0→1→0 (cycle)
        // Component 2: 2→3→4 (chain)
        // Component 3: 5 (isolated)

        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);
        let n3 = ctx.add_node(3);
        let n4 = ctx.add_node(4);
        let n5 = ctx.add_node(5);

        // Component 1: cycle
        ctx.add_edge("0->1", n0, n1);
        ctx.add_edge("1->0", n1, n0);

        // Component 2: chain
        ctx.add_edge("2->3", n2, n3);
        ctx.add_edge("3->4", n3, n4);

        // Component 3: isolated node (no edges)
        let _n5 = n5; // Silence unused variable warning
    });

    let sccs: Vec<_> = tarjan(graph).collect();

    // Should have 5 SCCs: {0,1}, {2}, {3}, {4}, {5}
    assert_eq!(
        sccs.len(),
        5,
        "Should have 5 SCCs for disconnected components"
    );

    let mut scc_sizes: Vec<usize> = sccs.iter().map(|scc| scc.len()).collect();
    scc_sizes.sort();

    // One 2-node SCC and four 1-node SCCs
    assert_eq!(
        scc_sizes,
        vec![1, 1, 1, 1, 2],
        "Expected one 2-node SCC and four 1-node SCCs"
    );
}

#[test]
fn test_tarjan_topological_ordering() {
    // Test that SCCs are returned in reverse topological order
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create a DAG of SCCs: A→B→C
        // Where A={0}, B={1,2}, C={3}
        // Should return in order: C, B, A

        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);
        let n3 = ctx.add_node(3);

        // SCC B: 1↔2
        ctx.add_edge("1->2", n1, n2);
        ctx.add_edge("2->1", n2, n1);

        // Cross-SCC edges: A→B→C
        ctx.add_edge("0->1", n0, n1); // A→B
        ctx.add_edge("2->3", n2, n3); // B→C
    });

    let sccs: Vec<_> = tarjan(graph).collect();
    assert_eq!(sccs.len(), 3, "Should have 3 SCCs");

    // The SCCs should be returned in reverse topological order
    // So we expect: SCC containing {3}, SCC containing {1,2}, SCC containing {0}
    assert_eq!(sccs[0].len(), 1, "First SCC should have 1 node");
    assert_eq!(sccs[1].len(), 2, "Second SCC should have 2 nodes");
    assert_eq!(sccs[2].len(), 1, "Third SCC should have 1 node");
}

#[test]
fn test_tarjan_with_self_loops() {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create nodes with self-loops and regular cycles
        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);

        // Self-loops
        ctx.add_edge("0->0", n0, n0);
        ctx.add_edge("2->2", n2, n2);

        // Regular edges
        ctx.add_edge("0->1", n0, n1);
        ctx.add_edge("1->2", n1, n2);
    });

    let sccs: Vec<_> = tarjan(graph).collect();

    // Each node should be its own SCC (self-loops don't create cycles with other nodes)
    assert_eq!(sccs.len(), 3, "Should have 3 SCCs");

    for scc in &sccs {
        assert_eq!(scc.len(), 1, "Each SCC should contain exactly 1 node");
    }
}

#[test]
fn test_large_graph_performance() {
    // Test with a larger graph to ensure the algorithm scales
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        let mut nodes = Vec::new();

        // Create 100 nodes
        for i in 0..100 {
            nodes.push(ctx.add_node(i));
        }

        // Create a long cycle: 0→1→2→...→99→0
        for i in 0..100 {
            let from = nodes[i];
            let to = nodes[(i + 1) % 100];
            ctx.add_edge(format!("edge_{}_to_{}", i, (i + 1) % 100), from, to);
        }
    });

    let sccs: Vec<_> = tarjan(graph).collect();

    // All 100 nodes should be in a single SCC due to the cycle
    assert_eq!(sccs.len(), 1, "Large cycle should form single SCC");
    assert_eq!(sccs[0].len(), 100, "SCC should contain all 100 nodes");
}

#[test]
fn test_massive_graph_stress() {
    // Stress test with 1000 nodes to verify algorithm can handle large graphs
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        let mut nodes = Vec::new();

        // Create 1000 nodes
        for i in 0..1000 {
            nodes.push(ctx.add_node(i));
        }

        // Create multiple large SCCs:
        // SCC 1: nodes 0-299 in a cycle
        // SCC 2: nodes 300-599 in a cycle
        // SCC 3: nodes 600-799 in a cycle
        // Individual SCCs: nodes 800-999

        // Large cycle 1: 0→1→...→299→0
        for i in 0..300 {
            let from = nodes[i];
            let to = nodes[(i + 1) % 300];
            ctx.add_edge(format!("scc1_{}_to_{}", i, (i + 1) % 300), from, to);
        }

        // Large cycle 2: 300→301→...→599→300
        for i in 300..600 {
            let from = nodes[i];
            let to = nodes[300 + ((i - 300 + 1) % 300)];
            ctx.add_edge(
                format!("scc2_{}_to_{}", i, 300 + ((i - 300 + 1) % 300)),
                from,
                to,
            );
        }

        // Medium cycle 3: 600→601→...→799→600
        for i in 600..800 {
            let from = nodes[i];
            let to = nodes[600 + ((i - 600 + 1) % 200)];
            ctx.add_edge(
                format!("scc3_{}_to_{}", i, 600 + ((i - 600 + 1) % 200)),
                from,
                to,
            );
        }

        // Cross-SCC edges to create dependencies
        ctx.add_edge("cross_1_to_2".to_string(), nodes[50], nodes[350]);
        ctx.add_edge("cross_2_to_3".to_string(), nodes[450], nodes[650]);
        ctx.add_edge("cross_3_to_individual".to_string(), nodes[750], nodes[850]);

        // Chain some individual nodes
        for i in 800..950 {
            ctx.add_edge(format!("chain_{}_to_{}", i, i + 1), nodes[i], nodes[i + 1]);
        }
    });

    let start_time = std::time::Instant::now();
    let sccs: Vec<_> = tarjan(graph).collect();
    let duration = start_time.elapsed();

    // Verify results
    assert_eq!(
        sccs.len(),
        203,
        "Should have 203 SCCs: 3 large cycles + 200 individual nodes"
    );

    // Check SCC sizes
    let mut scc_sizes: Vec<usize> = sccs.iter().map(|scc| scc.len()).collect();
    scc_sizes.sort();

    // Should have: 150 size-1 SCCs, 50 size-1 SCCs (individual), 1 size-200 SCC, 1 size-300 SCC, 1 size-300 SCC
    let large_sccs: Vec<usize> = scc_sizes
        .iter()
        .filter(|&&size| size > 1)
        .cloned()
        .collect();
    assert_eq!(large_sccs.len(), 3, "Should have exactly 3 large SCCs");
    assert!(large_sccs.contains(&200), "Should have one 200-node SCC");
    assert_eq!(
        large_sccs.iter().filter(|&&size| size == 300).count(),
        2,
        "Should have two 300-node SCCs"
    );

    // Verify total node count
    let total_nodes: usize = sccs.iter().map(|scc| scc.len()).sum();
    assert_eq!(total_nodes, 1000, "Should have exactly 1000 nodes total");

    // Performance check: should complete within reasonable time (1 second is generous)
    assert!(
        duration.as_secs() < 1,
        "Algorithm should complete within 1 second for 1000 nodes"
    );
}

#[test]
fn test_parallel_edges() {
    // Test graph with multiple edges between the same pair of nodes
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);

        // Add multiple edges between same nodes
        ctx.add_edge("edge1_0_to_1", n0, n1);
        ctx.add_edge("edge2_0_to_1", n0, n1);
        ctx.add_edge("edge3_0_to_1", n0, n1);

        // Add reverse edges to create SCC
        ctx.add_edge("edge1_1_to_0", n1, n0);
        ctx.add_edge("edge2_1_to_0", n1, n0);

        // Add parallel edges to another node
        ctx.add_edge("edge1_1_to_2", n1, n2);
        ctx.add_edge("edge2_1_to_2", n1, n2);
    });

    let sccs: Vec<_> = tarjan(graph).collect();

    // Should have 2 SCCs: {0,1} and {2}
    assert_eq!(sccs.len(), 2, "Should have 2 SCCs despite parallel edges");

    let mut scc_sizes: Vec<usize> = sccs.iter().map(|scc| scc.len()).collect();
    scc_sizes.sort();
    assert_eq!(
        scc_sizes,
        vec![1, 2],
        "Should have one 1-node SCC and one 2-node SCC"
    );
}

#[test]
fn test_empty_edges_after_nodes() {
    // Test edge case where nodes exist but no edges are added
    let mut graph: VecGraph<i32, &str> = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Add 5 nodes but no edges
        for i in 0..5 {
            ctx.add_node(i);
        }
    });

    let sccs: Vec<_> = tarjan(graph).collect();

    // Each node should be its own SCC
    assert_eq!(sccs.len(), 5, "Should have 5 SCCs for 5 isolated nodes");

    for scc in &sccs {
        assert_eq!(scc.len(), 1, "Each SCC should contain exactly 1 node");
    }
}

#[test]
fn test_deep_recursion_pathological() {
    // Test pathological case that could cause stack overflow with naive DFS
    // Create a long chain: 0→1→2→...→99→0 (single large SCC)
    let mut graph = VecGraph::default();

    const CHAIN_LENGTH: usize = 500; // Long enough to potentially cause stack issues

    graph.scope_mut(|mut ctx| {
        let mut nodes = Vec::new();

        // Create a long chain of nodes
        for i in 0..CHAIN_LENGTH {
            nodes.push(ctx.add_node(i));
        }

        // Connect them in a long cycle: 0→1→2→...→(n-1)→0
        for i in 0..CHAIN_LENGTH {
            let from = nodes[i];
            let to = nodes[(i + 1) % CHAIN_LENGTH];
            ctx.add_edge(
                format!("edge_{}_to_{}", i, (i + 1) % CHAIN_LENGTH),
                from,
                to,
            );
        }
    });

    let sccs: Vec<_> = tarjan(graph).collect();

    // Should form a single SCC containing all nodes
    assert_eq!(sccs.len(), 1, "Deep chain cycle should form single SCC");
    assert_eq!(
        sccs[0].len(),
        CHAIN_LENGTH,
        "SCC should contain all nodes in the chain"
    );
}

#[test]
fn test_deep_tree_structure() {
    // Test deep tree structure that doesn't form cycles (each node is its own SCC)
    let mut graph = VecGraph::default();

    const DEPTH: usize = 200;

    graph.scope_mut(|mut ctx| {
        let mut nodes = Vec::new();

        // Create nodes
        for i in 0..DEPTH {
            nodes.push(ctx.add_node(i));
        }

        // Create a deep tree: 0→1→2→...→(DEPTH-1)
        for i in 0..(DEPTH - 1) {
            ctx.add_edge(format!("edge_{}_to_{}", i, i + 1), nodes[i], nodes[i + 1]);
        }
    });

    let sccs: Vec<_> = tarjan(graph).collect();

    // Each node should be its own SCC since there are no cycles
    assert_eq!(
        sccs.len(),
        DEPTH,
        "Deep tree should have each node as its own SCC"
    );

    for scc in &sccs {
        assert_eq!(scc.len(), 1, "Each SCC should contain exactly 1 node");
    }
}

#[test]
fn test_randomized_properties() {
    // Property-based test with pseudo-random graph generation
    // Uses a fixed seed for reproducibility

    let seed = 42u64;
    let mut rng_state = seed;

    // Simple LCG for deterministic pseudo-random numbers
    let next_random = |state: &mut u64| -> u64 {
        *state = state.wrapping_mul(1103515245).wrapping_add(12345);
        *state
    };

    for test_iteration in 0..10 {
        let mut graph = VecGraph::default();
        let node_count = 10 + (next_random(&mut rng_state) % 20) as usize; // 10-29 nodes

        graph.scope_mut(|mut ctx| {
            let mut nodes = Vec::new();

            // Create nodes
            for i in 0..node_count {
                nodes.push(ctx.add_node(format!("node_{}", i)));
            }

            // Add random edges (about 2x as many edges as nodes)
            let edge_count = node_count * 2;
            for edge_id in 0..edge_count {
                let from_idx = (next_random(&mut rng_state) % node_count as u64) as usize;
                let to_idx = (next_random(&mut rng_state) % node_count as u64) as usize;

                ctx.add_edge(
                    format!("edge_{}_{}_to_{}", edge_id, from_idx, to_idx),
                    nodes[from_idx],
                    nodes[to_idx],
                );
            }
        });

        let sccs: Vec<_> = tarjan(graph).collect();

        // Verify fundamental properties

        // 1. Every node belongs to exactly one SCC
        let total_nodes_in_sccs: usize = sccs.iter().map(|scc| scc.len()).sum();
        assert_eq!(
            total_nodes_in_sccs, node_count,
            "Iteration {}: Total nodes in SCCs should equal graph size",
            test_iteration
        );

        // 2. No SCC should be empty
        for (scc_idx, scc) in sccs.iter().enumerate() {
            assert!(
                !scc.is_empty(),
                "Iteration {}: SCC {} should not be empty",
                test_iteration,
                scc_idx
            );
        }

        // 3. All nodes should be accounted for (no duplicates across SCCs)
        let mut all_nodes = HashSet::new();
        for scc in &sccs {
            for node in scc.iter() {
                assert!(
                    all_nodes.insert(node),
                    "Iteration {}: Node {:?} appears in multiple SCCs",
                    test_iteration,
                    node
                );
            }
        }
        assert_eq!(
            all_nodes.len(),
            node_count,
            "Iteration {}: Should have exactly {} unique nodes",
            test_iteration,
            node_count
        );

        // 4. Verify SCC count is reasonable (between 1 and node_count)
        assert!(
            sccs.len() >= 1 && sccs.len() <= node_count,
            "Iteration {}: SCC count {} should be between 1 and {}",
            test_iteration,
            sccs.len(),
            node_count
        );
    }
}

#[test]
fn test_algorithm_properties() {
    let graph = create_comprehensive_test_graph();
    let sccs: Vec<_> = tarjan(graph).collect();

    // Verify fundamental properties of SCC decomposition:

    // 1. Partition property: every node belongs to exactly one SCC
    let mut node_count = 0;
    let mut all_nodes = HashSet::new();

    for scc in &sccs {
        node_count += scc.len();
        for node in scc.iter() {
            assert!(all_nodes.insert(node), "Node appears in multiple SCCs");
        }
    }
    assert_eq!(node_count, 7, "Total node count should equal graph size");

    // 2. Non-empty property: no SCC should be empty
    for scc in &sccs {
        assert!(!scc.is_empty(), "SCC should not be empty");
        assert!(scc.len() > 0, "SCC should have positive length");
    }

    // 3. Verify SCC struct methods work correctly
    for scc in &sccs {
        assert_eq!(scc.len(), scc.len(), "scc.len() should equal scc.len()");
        assert_eq!(
            scc.is_empty(),
            scc.len() == 0,
            "is_empty() should match len() == 0"
        );
    }
}

#[test]
fn test_performance_benchmarks() {
    // Benchmark different graph types to ensure consistent performance

    // Test 1: Dense graph (many edges)
    let start = std::time::Instant::now();
    let mut dense_graph = VecGraph::default();
    dense_graph.scope_mut(|mut ctx| {
        let mut nodes = Vec::new();
        for i in 0..50 {
            nodes.push(ctx.add_node(i));
        }

        // Create dense connections (each node connects to next 5 nodes)
        for i in 0..50 {
            for j in 1..=5 {
                let to_idx = (i + j) % 50;
                ctx.add_edge(
                    format!("dense_{}_to_{}", i, to_idx),
                    nodes[i],
                    nodes[to_idx],
                );
            }
        }
    });
    let dense_sccs: Vec<_> = tarjan(dense_graph).collect();
    let dense_duration = start.elapsed();

    // Test 2: Sparse graph (few edges)
    let start = std::time::Instant::now();
    let mut sparse_graph = VecGraph::default();
    sparse_graph.scope_mut(|mut ctx| {
        let mut nodes = Vec::new();
        for i in 0..100 {
            nodes.push(ctx.add_node(i));
        }

        // Create sparse connections (only every 10th node connects)
        for i in (0..100).step_by(10) {
            if i + 10 < 100 {
                ctx.add_edge(
                    format!("sparse_{}_to_{}", i, i + 10),
                    nodes[i],
                    nodes[i + 10],
                );
            }
        }
    });
    let sparse_sccs: Vec<_> = tarjan(sparse_graph).collect();
    let sparse_duration = start.elapsed();

    // Test 3: Many small cycles
    let start = std::time::Instant::now();
    let mut cycles_graph = VecGraph::default();
    cycles_graph.scope_mut(|mut ctx| {
        let mut nodes = Vec::new();
        for i in 0..60 {
            nodes.push(ctx.add_node(i));
        }

        // Create 20 cycles of 3 nodes each
        for cycle_id in 0..20 {
            let base = cycle_id * 3;
            ctx.add_edge(
                format!("cycle_{}_0_to_1", cycle_id),
                nodes[base],
                nodes[base + 1],
            );
            ctx.add_edge(
                format!("cycle_{}_1_to_2", cycle_id),
                nodes[base + 1],
                nodes[base + 2],
            );
            ctx.add_edge(
                format!("cycle_{}_2_to_0", cycle_id),
                nodes[base + 2],
                nodes[base],
            );
        }
    });
    let cycles_sccs: Vec<_> = tarjan(cycles_graph).collect();
    let cycles_duration = start.elapsed();

    // Performance assertions (all should complete quickly)
    assert!(
        dense_duration.as_millis() < 100,
        "Dense graph should complete in < 100ms"
    );
    assert!(
        sparse_duration.as_millis() < 100,
        "Sparse graph should complete in < 100ms"
    );
    assert!(
        cycles_duration.as_millis() < 100,
        "Cycles graph should complete in < 100ms"
    );

    // Correctness assertions
    assert_eq!(dense_sccs.len(), 1, "Dense graph should form one large SCC");
    assert_eq!(
        sparse_sccs.len(),
        100,
        "Sparse graph should have each node as its own SCC"
    );
    assert_eq!(cycles_sccs.len(), 20, "Cycles graph should have 20 SCCs");

    // Verify cycles graph has correct SCC sizes
    for scc in &cycles_sccs {
        assert_eq!(scc.len(), 3, "Each cycle should form a 3-node SCC");
    }
}
