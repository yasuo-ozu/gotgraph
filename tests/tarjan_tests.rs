use gotgraph::algo::tarjan;
use gotgraph::prelude::*;

/// Create a simple test graph with no cycles
fn create_linear_graph() -> VecGraph<i32, &'static str> {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create nodes: 0 -> 1 -> 2 -> 3
        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);
        let n3 = ctx.add_node(3);

        ctx.add_edge("0->1", n0, n1);
        ctx.add_edge("1->2", n1, n2);
        ctx.add_edge("2->3", n2, n3);
    });

    graph
}

/// Create a graph with a simple cycle
fn create_cycle_graph() -> VecGraph<i32, &'static str> {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create cycle: 0 -> 1 -> 2 -> 0
        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);

        ctx.add_edge("0->1", n0, n1);
        ctx.add_edge("1->2", n1, n2);
        ctx.add_edge("2->0", n2, n0);
    });

    graph
}

/// Create a complex graph with multiple SCCs
fn create_complex_graph() -> VecGraph<i32, &'static str> {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Create multiple SCCs:
        // SCC1: 0 <-> 1 (2-node cycle)
        // SCC2: 2 -> 3 -> 4 -> 2 (3-node cycle)
        // SCC3: 5 (isolated node)
        // Cross-SCC edges: 1 -> 2, 4 -> 5

        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        let n2 = ctx.add_node(2);
        let n3 = ctx.add_node(3);
        let n4 = ctx.add_node(4);
        let n5 = ctx.add_node(5);

        // SCC1: 0 <-> 1
        ctx.add_edge("0->1", n0, n1);
        ctx.add_edge("1->0", n1, n0);

        // SCC2: 2 -> 3 -> 4 -> 2
        ctx.add_edge("2->3", n2, n3);
        ctx.add_edge("3->4", n3, n4);
        ctx.add_edge("4->2", n4, n2);

        // Cross-SCC edges
        ctx.add_edge("1->2", n1, n2);
        ctx.add_edge("4->5", n4, n5);
    });

    graph
}

#[test]
fn test_empty_graph() {
    let graph = VecGraph::<i32, &str>::default();
    let sccs: Vec<_> = tarjan(graph).collect();
    assert_eq!(sccs.len(), 0);
}

#[test]
fn test_single_node() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        ctx.add_node(42);
    });

    let sccs: Vec<_> = tarjan(graph).collect();
    assert_eq!(sccs.len(), 1);
    assert_eq!(sccs[0].len(), 1);
    // Just check that the SCC contains one node (don't check the exact index)
}

#[test]
fn test_linear_graph_no_cycles() {
    let graph = create_linear_graph();
    let sccs: Vec<_> = tarjan(graph).collect();

    // Each node should be its own SCC
    assert_eq!(sccs.len(), 4);

    // Each SCC should contain exactly one node
    for scc in &sccs {
        assert_eq!(scc.len(), 1);
    }
}

#[test]
fn test_simple_cycle() {
    let graph = create_cycle_graph();
    let sccs: Vec<_> = tarjan(graph).collect();

    // Should have one SCC containing all 3 nodes
    assert_eq!(sccs.len(), 1);
    assert_eq!(sccs[0].len(), 3);

    // All nodes should be in the same SCC
    let node_indices: std::collections::HashSet<_> = sccs[0].iter().cloned().collect();
    assert_eq!(node_indices.len(), 3);
}

#[test]
fn test_self_loop() {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        let n0 = ctx.add_node(0);
        ctx.add_edge("self", n0, n0);
    });

    let sccs: Vec<_> = tarjan(graph).collect();
    assert_eq!(sccs.len(), 1);
    assert_eq!(sccs[0].len(), 1);
}

#[test]
fn test_complex_graph_multiple_sccs() {
    let graph = create_complex_graph();
    let sccs: Vec<_> = tarjan(graph).collect();

    // Should have 3 SCCs
    assert_eq!(sccs.len(), 3);

    // Sort SCCs by size for easier testing
    let mut scc_sizes: Vec<_> = sccs.iter().map(|scc| scc.len()).collect();
    scc_sizes.sort();

    // Expect: one 1-node SCC, one 2-node SCC, one 3-node SCC
    assert_eq!(scc_sizes, vec![1, 2, 3]);
}

#[test]
fn test_two_separate_cycles() {
    let mut graph = VecGraph::default();

    graph.scope_mut(|mut ctx| {
        // Cycle 1: 0 <-> 1
        let n0 = ctx.add_node(0);
        let n1 = ctx.add_node(1);
        ctx.add_edge("0->1", n0, n1);
        ctx.add_edge("1->0", n1, n0);

        // Cycle 2: 2 <-> 3
        let n2 = ctx.add_node(2);
        let n3 = ctx.add_node(3);
        ctx.add_edge("2->3", n2, n3);
        ctx.add_edge("3->2", n3, n2);
    });

    let sccs: Vec<_> = tarjan(graph).collect();

    // Should have 2 SCCs, each with 2 nodes
    assert_eq!(sccs.len(), 2);
    assert_eq!(sccs[0].len(), 2);
    assert_eq!(sccs[1].len(), 2);
}

#[test]
fn test_scc_box_slice_properties() {
    // Test that Box<[NodeIx]> behaves as expected
    let graph = create_cycle_graph();
    let sccs: Vec<_> = tarjan(graph).collect();

    // Test the first SCC (should contain all 3 nodes from the cycle)
    assert!(!sccs.is_empty());
    let scc = &sccs[0];

    assert_eq!(scc.len(), 3);
    assert!(!scc.is_empty());

    // Test empty box slice
    let empty_scc: Box<[gotgraph::vec_graph::NodeIx]> = vec![].into_boxed_slice();
    assert!(empty_scc.is_empty());
    assert_eq!(empty_scc.len(), 0);
}
