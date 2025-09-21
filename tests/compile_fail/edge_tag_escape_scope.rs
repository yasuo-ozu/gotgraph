use gotgraph::prelude::*;

fn main() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();
    
    // Try to move edge tags outside of scope - should fail
    let escaped_edge = graph.scope_mut(|mut ctx| {
        let n0 = ctx.add_node(1);
        let n1 = ctx.add_node(2);
        let edge = ctx.add_edge("test", n0, n1);
        edge // ERROR: edge tag cannot escape the scope
    });
    
    // Try to use the escaped edge tag - should fail
    println!("{:?}", escaped_edge);
}