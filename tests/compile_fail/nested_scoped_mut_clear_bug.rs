use gotgraph::prelude::*;

fn main() {
    let mut graph: VecGraph<i32, &str> = VecGraph::default();
    
    // Outer scope_mut - add some nodes and edges
    graph.scope_mut(|mut outer_ctx| {
        let node1 = outer_ctx.add_node(1);
        let node2 = outer_ctx.add_node(2);
        let edge1 = outer_ctx.add_edge("edge1", node1, node2);
        
        // This is the bug! We can call scope_mut on contexts, which creates nested scoping
        outer_ctx.scope_mut(|mut inner_ctx| {
            // In the inner scope, we clear all nodes and edges
            // This should invalidate the outer scope's references but doesn't prevent compilation
            let all_nodes: Vec<_> = inner_ctx.node_indices().collect();
            let all_edges: Vec<_> = inner_ctx.edge_indices().collect();
            
            // Try alternative ways to clear the graph from nested context
            // Method 1: Try to replace the entire graph with a new empty one
            // This should fail because we can't access the underlying graph directly
            // let empty_graph = VecGraph::default();
            // std::mem::replace(&mut inner_ctx, empty_graph); // Won't work - type mismatch
            
            // Method 2: Try to clear through the graph field (but it's private)
            // inner_ctx.graph.clear(); // Won't work - field is private
            
            // The fact that we can't easily clear shows the library has some protection,
            // but the nested scoping itself is still problematic
            let _node3 = inner_ctx.add_node(3);
            
            // This would be the dangerous operation if it worked:
            inner_ctx.remove_nodes_edges::<Vec<_>, Vec<_>>(all_nodes, all_edges);
        });
        
        // This is the potential bug: we can still access node1, node2, edge1 
        // even though the inner scope could have modified the graph structure
        // If the inner scope had removed these nodes, this would be use-after-free
        println!("Node1 value: {}", outer_ctx.node(node1)); // Potentially dangerous
        println!("Node2 value: {}", outer_ctx.node(node2)); // Potentially dangerous
        println!("Edge1 value: {}", outer_ctx.edge(edge1)); // Potentially dangerous
    });
}