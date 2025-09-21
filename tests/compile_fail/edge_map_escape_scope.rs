use gotgraph::prelude::*;

fn main() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();
    
    graph.scope_mut(|mut ctx| {
        let n0 = ctx.add_node(1);
        let n1 = ctx.add_node(2);
        ctx.add_edge("test", n0, n1);
    });
    
    // Try to move edge map outside of scope - should fail
    let escaped_map = graph.scope(|ctx| {
        let edge_map = ctx.init_edge_map(|_, edge| edge.len());
        edge_map // ERROR: edge map cannot escape the scope
    });
    
    // Try to use the escaped map - should fail
    let _ = escaped_map;
}