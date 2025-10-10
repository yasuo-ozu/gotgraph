use rand::prelude::*;

// Import gotgraph
use gotgraph::prelude::*;

// Import petgraph
use petgraph::graph::DiGraph;
use petgraph::stable_graph::StableDiGraph;

// Import other graph libraries
use graphlib::Graph as GraphlibGraph;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub graph_size: usize,
    pub gotgraph_scoped_time_ns: u64,
    pub gotgraph_direct_time_ns: u64,
    pub petgraph_time_ns: u64,
    pub petgraph_stable_time_ns: u64,
    pub pathfinding_time_ns: u64,
    pub graphlib_time_ns: u64,
}

pub fn generate_random_edges(num_nodes: usize, num_edges: usize, rng: &mut StdRng) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for _ in 0..num_edges {
        let from = rng.gen_range(0..num_nodes);
        let to = rng.gen_range(0..num_nodes);
        edges.push((from, to));
    }
    edges
}

/// Benchmark gotgraph scoped graph creation
pub fn benchmark_gotgraph_scoped_creation(
    num_nodes: usize,
    edges: &[(usize, usize)],
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut graph: VecGraph<usize, usize> = VecGraph::default();
        graph.scope_mut(|mut ctx| {
            let node_tags: Vec<_> = (0..num_nodes)
                .map(|i| ctx.add_node(i))
                .collect();
            for (edge_idx, &(from, to)) in edges.iter().enumerate() {
                ctx.add_edge(edge_idx, node_tags[from], node_tags[to]);
            }
        });
    }
    start.elapsed()
}

/// Benchmark gotgraph direct graph creation
pub fn benchmark_gotgraph_direct_creation(
    num_nodes: usize,
    edges: &[(usize, usize)],
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut graph: VecGraph<usize, usize> = VecGraph::default();
        let node_indices: Vec<_> = (0..num_nodes)
            .map(|i| graph.add_node(i))
            .collect();
        for (edge_idx, &(from, to)) in edges.iter().enumerate() {
            graph.add_edge(edge_idx, node_indices[from], node_indices[to]);
        }
    }
    start.elapsed()
}

/// Benchmark petgraph DiGraph creation
pub fn benchmark_petgraph_creation(
    num_nodes: usize,
    edges: &[(usize, usize)],
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut graph = DiGraph::new();
        let node_indices: Vec<_> = (0..num_nodes)
            .map(|i| graph.add_node(i))
            .collect();
        for (edge_idx, &(from, to)) in edges.iter().enumerate() {
            graph.add_edge(node_indices[from], node_indices[to], edge_idx);
        }
    }
    start.elapsed()
}

/// Benchmark petgraph StableDiGraph creation
pub fn benchmark_petgraph_stable_creation(
    num_nodes: usize,
    edges: &[(usize, usize)],
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut graph = StableDiGraph::new();
        let node_indices: Vec<_> = (0..num_nodes)
            .map(|i| graph.add_node(i))
            .collect();
        for (edge_idx, &(from, to)) in edges.iter().enumerate() {
            graph.add_edge(node_indices[from], node_indices[to], edge_idx);
        }
    }
    start.elapsed()
}


/// Benchmark pathfinding graph creation (using adjacency lists)
pub fn benchmark_pathfinding_creation(
    num_nodes: usize,
    edges: &[(usize, usize)],
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut adjacency_list: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        for i in 0..num_nodes {
            adjacency_list.insert(i, Vec::new());
        }
        for (edge_idx, &(from, to)) in edges.iter().enumerate() {
            adjacency_list.get_mut(&from).unwrap().push((to, edge_idx));
        }
    }
    start.elapsed()
}

/// Benchmark graphlib creation
pub fn benchmark_graphlib_creation(
    num_nodes: usize,
    edges: &[(usize, usize)],
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut graph = GraphlibGraph::new();
        let mut vertex_ids = Vec::new();
        for i in 0..num_nodes {
            let id = graph.add_vertex(i);
            vertex_ids.push(id);
        }
        for &(from, to) in edges.iter() {
            graph.add_edge(&vertex_ids[from], &vertex_ids[to]).ok(); // Ignore errors for DAG constraints
        }
    }
    start.elapsed()
}


/// Benchmark gotgraph scoped traversal
pub fn benchmark_gotgraph_scoped_traversal(
    graph: &VecGraph<usize, usize>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    let mut global_total: usize = 0;
    for _ in 0..iterations {
        let total = graph.scope(|ctx| {
            let mut total = 0;
            for node_tag in ctx.node_indices() {
                let node_value = *ctx.node(node_tag);
                total += node_value;
                for edge_tag in ctx.outgoing_edge_indices(node_tag) {
                    let edge_value = *ctx.edge(edge_tag);
                    total += edge_value;
                }
            }
            total
        });
        global_total = global_total.wrapping_add(total);
    }
    // Use the total to prevent optimization
    std::hint::black_box(global_total);
    start.elapsed()
}

/// Benchmark gotgraph direct traversal
pub fn benchmark_gotgraph_direct_traversal(
    graph: &VecGraph<usize, usize>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    let mut global_total: usize = 0;
    for _ in 0..iterations {
        let mut total = 0;
        for node_idx in graph.node_indices() {
            let node_value = *graph.node(node_idx);
            total += node_value;
            for edge_idx in graph.outgoing_edge_indices(node_idx) {
                let edge_value = *graph.edge(edge_idx);
                total += edge_value;
            }
        }
        global_total = global_total.wrapping_add(total);
    }
    // Use the total to prevent optimization
    std::hint::black_box(global_total);
    start.elapsed()
}

/// Benchmark petgraph DiGraph traversal
pub fn benchmark_petgraph_traversal(
    graph: &DiGraph<usize, usize>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    let mut global_total: usize = 0;
    for _ in 0..iterations {
        let mut total = 0;
        for node_idx in graph.node_indices() {
            let node_value = *graph.node_weight(node_idx).unwrap();
            total += node_value;
            for edge_ref in graph.edges(node_idx) {
                let edge_value = *edge_ref.weight();
                total += edge_value;
            }
        }
        global_total = global_total.wrapping_add(total);
    }
    // Use the total to prevent optimization
    std::hint::black_box(global_total);
    start.elapsed()
}

/// Benchmark petgraph StableDiGraph traversal
pub fn benchmark_petgraph_stable_traversal(
    graph: &StableDiGraph<usize, usize>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    let mut global_total: usize = 0;
    for _ in 0..iterations {
        let mut total = 0;
        for node_idx in graph.node_indices() {
            let node_value = *graph.node_weight(node_idx).unwrap();
            total += node_value;
            for edge_ref in graph.edges(node_idx) {
                let edge_value = *edge_ref.weight();
                total += edge_value;
            }
        }
        global_total = global_total.wrapping_add(total);
    }
    // Use the total to prevent optimization
    std::hint::black_box(global_total);
    start.elapsed()
}


/// Benchmark pathfinding traversal (using adjacency lists)
pub fn benchmark_pathfinding_traversal(
    adjacency_list: &HashMap<usize, Vec<(usize, usize)>>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    let mut global_total: usize = 0;
    for _ in 0..iterations {
        let mut total = 0;
        for (&node, edges) in adjacency_list.iter() {
            total += node;
            for &(_, edge_weight) in edges.iter() {
                total += edge_weight;
            }
        }
        global_total = global_total.wrapping_add(total);
    }
    // Use the total to prevent optimization
    std::hint::black_box(global_total);
    start.elapsed()
}

/// Benchmark graphlib traversal
pub fn benchmark_graphlib_traversal(
    graph: &GraphlibGraph<usize>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    let mut global_total: usize = 0;
    for _ in 0..iterations {
        let mut total = 0;
        
        // Iterate over all vertices in the graph
        for vertex_id in graph.vertices() {
            // Get the vertex value (similar to gotgraph's node access)
            if let Some(vertex_value) = graph.fetch(&vertex_id) {
                total += vertex_value;
            }
            
            // Iterate over outgoing neighbors (similar to gotgraph's outgoing edges)
            for neighbor_id in graph.out_neighbors(&vertex_id) {
                // Access neighbor value to simulate edge traversal work
                if let Some(neighbor_value) = graph.fetch(&neighbor_id) {
                    total += neighbor_value;
                }
            }
        }
        
        global_total = global_total.wrapping_add(total);
    }
    // Use the total to prevent optimization
    std::hint::black_box(global_total);
    start.elapsed()
}


/// Create test graphs for benchmarking
pub fn create_test_graphs(
    num_nodes: usize,
    edges: &[(usize, usize)],
) -> (VecGraph<usize, usize>, DiGraph<usize, usize>, StableDiGraph<usize, usize>, HashMap<usize, Vec<(usize, usize)>>, GraphlibGraph<usize>) {
    // Create gotgraph
    let mut gotgraph_graph: VecGraph<usize, usize> = VecGraph::default();
    gotgraph_graph.scope_mut(|mut ctx| {
        let node_tags: Vec<_> = (0..num_nodes)
            .map(|i| ctx.add_node(i))
            .collect();
        for (edge_idx, &(from, to)) in edges.iter().enumerate() {
            ctx.add_edge(edge_idx, node_tags[from], node_tags[to]);
        }
    });

    // Create petgraph DiGraph
    let mut petgraph_graph = DiGraph::new();
    let petgraph_nodes: Vec<_> = (0..num_nodes)
        .map(|i| petgraph_graph.add_node(i))
        .collect();
    for (edge_idx, &(from, to)) in edges.iter().enumerate() {
        petgraph_graph.add_edge(petgraph_nodes[from], petgraph_nodes[to], edge_idx);
    }

    // Create petgraph StableDiGraph
    let mut stable_graph = StableDiGraph::new();
    let stable_nodes: Vec<_> = (0..num_nodes)
        .map(|i| stable_graph.add_node(i))
        .collect();
    for (edge_idx, &(from, to)) in edges.iter().enumerate() {
        stable_graph.add_edge(stable_nodes[from], stable_nodes[to], edge_idx);
    }

    // Create pathfinding adjacency list
    let mut adjacency_list: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..num_nodes {
        adjacency_list.insert(i, Vec::new());
    }
    for (edge_idx, &(from, to)) in edges.iter().enumerate() {
        adjacency_list.get_mut(&from).unwrap().push((to, edge_idx));
    }

    // Create graphlib graph
    let mut graphlib_graph = GraphlibGraph::new();
    let mut vertex_ids = Vec::new();
    for i in 0..num_nodes {
        let id = graphlib_graph.add_vertex(i);
        vertex_ids.push(id);
    }
    for &(from, to) in edges.iter() {
        graphlib_graph.add_edge(&vertex_ids[from], &vertex_ids[to]).ok(); // Ignore errors for DAG constraints
    }

    (gotgraph_graph, petgraph_graph, stable_graph, adjacency_list, graphlib_graph)
}

/// Create test graphs for benchmarking with node indices as values
pub fn create_test_graphs_with_indices(
    num_nodes: usize,
    edges: &[(usize, usize)],
) -> (VecGraph<usize, usize>, DiGraph<usize, usize>, StableDiGraph<usize, usize>) {
    // Create gotgraph
    let mut gotgraph_graph: VecGraph<usize, usize> = VecGraph::default();
    gotgraph_graph.scope_mut(|mut ctx| {
        let node_tags: Vec<_> = (0..num_nodes)
            .map(|i| ctx.add_node(i))
            .collect();
        for (edge_idx, &(from, to)) in edges.iter().enumerate() {
            ctx.add_edge(edge_idx, node_tags[from], node_tags[to]);
        }
    });

    // Create petgraph DiGraph
    let mut petgraph_graph = DiGraph::new();
    let petgraph_nodes: Vec<_> = (0..num_nodes)
        .map(|i| petgraph_graph.add_node(i))
        .collect();
    for (edge_idx, &(from, to)) in edges.iter().enumerate() {
        petgraph_graph.add_edge(petgraph_nodes[from], petgraph_nodes[to], edge_idx);
    }

    // Create petgraph StableDiGraph
    let mut stable_graph = StableDiGraph::new();
    let stable_nodes: Vec<_> = (0..num_nodes)
        .map(|i| stable_graph.add_node(i))
        .collect();
    for (edge_idx, &(from, to)) in edges.iter().enumerate() {
        stable_graph.add_edge(stable_nodes[from], stable_nodes[to], edge_idx);
    }

    (gotgraph_graph, petgraph_graph, stable_graph)
}

/// Run comprehensive benchmark for a given graph size
pub fn run_comprehensive_benchmark(size: usize, iterations: usize) -> BenchmarkResult {
    let num_nodes = size;
    let num_edges = num_nodes * 2;
    
    let mut rng = StdRng::seed_from_u64(42);
    let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
    
    println!("Benchmarking size: {}", size);
    
    // Benchmark creation
    let gotgraph_scoped_time = benchmark_gotgraph_scoped_creation(num_nodes, &edges, iterations);
    let gotgraph_direct_time = benchmark_gotgraph_direct_creation(num_nodes, &edges, iterations);
    let petgraph_time = benchmark_petgraph_creation(num_nodes, &edges, iterations);
    let petgraph_stable_time = benchmark_petgraph_stable_creation(num_nodes, &edges, iterations);
    let pathfinding_time = benchmark_pathfinding_creation(num_nodes, &edges, iterations);
    let graphlib_time = benchmark_graphlib_creation(num_nodes, &edges, iterations);
    
    BenchmarkResult {
        graph_size: size,
        gotgraph_scoped_time_ns: gotgraph_scoped_time.as_nanos() as u64 / iterations as u64,
        gotgraph_direct_time_ns: gotgraph_direct_time.as_nanos() as u64 / iterations as u64,
        petgraph_time_ns: petgraph_time.as_nanos() as u64 / iterations as u64,
        petgraph_stable_time_ns: petgraph_stable_time.as_nanos() as u64 / iterations as u64,
        pathfinding_time_ns: pathfinding_time.as_nanos() as u64 / iterations as u64,
        graphlib_time_ns: graphlib_time.as_nanos() as u64 / iterations as u64,
    }
}

/// Print performance summary for a benchmark result
pub fn print_performance_summary(results: &[BenchmarkResult], operation: &str) {
    println!("\n{}:", operation);
    for result in results {
        let scoped_vs_petgraph = result.gotgraph_scoped_time_ns as f64 / result.petgraph_time_ns as f64;
        let direct_vs_petgraph = result.gotgraph_direct_time_ns as f64 / result.petgraph_time_ns as f64;
        let stable_vs_petgraph = result.petgraph_stable_time_ns as f64 / result.petgraph_time_ns as f64;
        let scoped_vs_direct = result.gotgraph_scoped_time_ns as f64 / result.gotgraph_direct_time_ns as f64;
        
        println!("  Size {}: GotGraph(S) {}ns, GotGraph(D) {}ns, PetGraph {}ns, PetStable {}ns",
                 result.graph_size, 
                 result.gotgraph_scoped_time_ns, 
                 result.gotgraph_direct_time_ns,
                 result.petgraph_time_ns, 
                 result.petgraph_stable_time_ns);
        println!("    Ratios vs PetGraph: Scoped {:.2}x, Direct {:.2}x, Stable {:.2}x | Scoped vs Direct {:.2}x",
                 scoped_vs_petgraph, direct_vs_petgraph, stable_vs_petgraph, scoped_vs_direct);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graphlib_traversal_implementation() {
        // Create a small test graph
        let mut graph = GraphlibGraph::new();
        
        // Add vertices
        let v1 = graph.add_vertex(10);
        let v2 = graph.add_vertex(20);
        let v3 = graph.add_vertex(30);
        
        // Add edges
        graph.add_edge(&v1, &v2).ok();
        graph.add_edge(&v2, &v3).ok();
        graph.add_edge(&v1, &v3).ok();
        
        println!("Created test graph with {} vertices", graph.vertices().count());
        
        // Test the traversal
        let duration = benchmark_graphlib_traversal(&graph, 1);
        println!("GraphLib traversal completed in {:?}", duration);
        
        // Test that our traversal actually iterates through the graph
        let mut total = 0;
        for vertex_id in graph.vertices() {
            if let Some(vertex_value) = graph.fetch(&vertex_id) {
                total += vertex_value;
                println!("Visited vertex with value: {}", vertex_value);
                
                for neighbor_id in graph.out_neighbors(&vertex_id) {
                    if let Some(neighbor_value) = graph.fetch(&neighbor_id) {
                        total += neighbor_value;
                        println!("  -> neighbor with value: {}", neighbor_value);
                    }
                }
            }
        }
        
        println!("Total sum from manual traversal: {}", total);
        assert!(total > 0, "Traversal should visit vertices and accumulate values");
        assert!(duration.as_nanos() > 0, "Benchmark should take some time");
    }
}