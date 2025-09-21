use gotgraph::prelude::*;

fn main() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();
    
    // Try to move node tags outside of scope - should fail
    let escaped_node = graph.scope_mut(|mut ctx| {
        let n0 = ctx.add_node(42);
        n0 // ERROR: node tag cannot escape the scope
    });
    
    // Try to use the escaped node tag - should fail
    println!("{:?}", escaped_node);
}