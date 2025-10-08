use rand::prelude::*;

// Import gotgraph
use gotgraph::prelude::*;

// Import petgraph
use petgraph::graph::DiGraph;
use petgraph::stable_graph::StableDiGraph;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub graph_size: usize,
    pub gotgraph_scoped_time_ns: u64,
    pub gotgraph_direct_time_ns: u64,
    pub petgraph_time_ns: u64,
    pub petgraph_stable_time_ns: u64,
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

/// Create test graphs for benchmarking
pub fn create_test_graphs(
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
    
    BenchmarkResult {
        graph_size: size,
        gotgraph_scoped_time_ns: gotgraph_scoped_time.as_nanos() as u64 / iterations as u64,
        gotgraph_direct_time_ns: gotgraph_direct_time.as_nanos() as u64 / iterations as u64,
        petgraph_time_ns: petgraph_time.as_nanos() as u64 / iterations as u64,
        petgraph_stable_time_ns: petgraph_stable_time.as_nanos() as u64 / iterations as u64,
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