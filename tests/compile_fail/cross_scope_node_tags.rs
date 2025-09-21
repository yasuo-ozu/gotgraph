use gotgraph::prelude::*;

fn main() {
    let mut graph1: VecGraph<i32, &str> = VecGraph::default();
    let mut graph2: VecGraph<i32, &str> = VecGraph::default();
    
    let node_from_graph1 = graph1.scope_mut(|mut ctx| {
        ctx.add_node(42)
    });
    
    // Try to use node tag from graph1 in graph2's scope - should fail
    graph2.scope_mut(|mut ctx| {
        let n1 = ctx.add_node(100);
        // ERROR: node_from_graph1 has wrong lifetime/scope
        ctx.add_edge("cross", node_from_graph1, n1);
    });
}