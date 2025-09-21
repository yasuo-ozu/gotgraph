use gotgraph::prelude::*;

fn main() {
    let mut outer_graph: VecGraph<i32, &str> = VecGraph::default();
    
    outer_graph.scope_mut(|mut outer_ctx| {
        let outer_node = outer_ctx.add_node(1);
        
        // Create inner graph within the scope
        let mut inner_graph: VecGraph<i32, &str> = VecGraph::default();
        
        inner_graph.scope_mut(|mut inner_ctx| {
            let inner_node = inner_ctx.add_node(2);
            
            // Try to mix tags from different scopes - should fail
            // ERROR: cannot use outer_node in inner context
            inner_ctx.add_edge("bad", outer_node, inner_node);
        });
    });
}