use gotgraph::prelude::*;

fn main() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();
    
    graph.scope_mut(|mut ctx| {
        ctx.add_node(1);
        ctx.add_node(2);
    });
    
    let saved_node_tag = graph.scope_mut(|mut ctx| {
        ctx.add_node(3)
    });
    
    // Try to use saved node tag in different scope - should fail
    graph.scope(|ctx| {
        let node_map = ctx.init_node_map(|_, &data| data);
        // ERROR: saved_node_tag from different scope
        let _value = node_map[saved_node_tag];
    });
}