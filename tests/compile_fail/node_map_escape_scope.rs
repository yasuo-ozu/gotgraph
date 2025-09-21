use gotgraph::prelude::*;

fn main() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();
    
    graph.scope_mut(|mut ctx| {
        ctx.add_node(1);
        ctx.add_node(2);
    });
    
    // Try to move node map outside of scope - should fail
    let escaped_map = graph.scope(|ctx| {
        let node_map = ctx.init_node_map(|_, &data| data * 2);
        node_map // ERROR: node map cannot escape the scope
    });
    
    // Try to use the escaped map - should fail
    let _ = escaped_map;
}