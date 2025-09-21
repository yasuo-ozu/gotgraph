use gotgraph::prelude::*;

fn main() {
    let mut graph1: VecGraph<i32, &str> = VecGraph::default();
    let mut graph2: VecGraph<i32, &str> = VecGraph::default();
    
    // Create nodes in different graphs
    let node1 = graph1.scope_mut(|mut ctx| {
        ctx.add_node(1)
    });
    
    let node2 = graph2.scope_mut(|mut ctx| {
        ctx.add_node(2)
    });
    
    // Try to create edge between nodes from different graphs - should fail
    graph1.scope_mut(|mut ctx| {
        // ERROR: node2 belongs to different graph/scope
        ctx.add_edge("invalid", node1, node2);
    });
}