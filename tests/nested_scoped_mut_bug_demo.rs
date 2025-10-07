// This test demonstrates a potential bug in the scoped_mut implementation
// It shows that nested scope_mut calls are allowed, which could be dangerous
// if the inner scope could remove nodes/edges while the outer scope holds references

use gotgraph::prelude::*;

#[test]
fn test_nested_scoped_mut_compiles() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();

    // Outer scope_mut - add some nodes and edges
    graph.scope_mut(|mut outer_ctx| {
        let node1 = outer_ctx.add_node(1);
        let node2 = outer_ctx.add_node(2);
        let edge1 = outer_ctx.add_edge("edge1", node1, node2);

        // BUG: We can call scope_mut on contexts, which creates nested scoping
        // This should be prevented to avoid potential use-after-free bugs
        outer_ctx.scope_mut(|mut inner_ctx| {
            // In the inner scope, we can add nodes, which affects the same underlying graph
            let _node3 = inner_ctx.add_node(3);

            // The dangerous part: if we could clear or remove nodes here,
            // the outer scope's references (node1, node2, edge1) would become invalid
            // But the borrow checker doesn't prevent accessing them later!
        });

        // This is the potential bug: we can still access node1, node2, edge1
        // even though the inner scope could have modified the graph structure
        // If the inner scope had removed these nodes, this would be use-after-free
        assert_eq!(*outer_ctx.node(node1), 1); // Potentially dangerous
        assert_eq!(*outer_ctx.node(node2), 2); // Potentially dangerous
        assert_eq!(*outer_ctx.edge(edge1), "edge1"); // Potentially dangerous
    });
}
