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
        let mut graph: VecGraph<usize, ()> = VecGraph::default();
        graph.scope_mut(|mut ctx| {
            let node_tags: Vec<_> = (0..num_nodes)
                .map(|i| ctx.add_node(i))
                .collect();
            for &(from, to) in edges {
                ctx.add_edge((), node_tags[from], node_tags[to]);
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
        let mut graph: VecGraph<usize, ()> = VecGraph::default();
        let node_indices: Vec<_> = (0..num_nodes)
            .map(|i| graph.add_node(i))
            .collect();
        for &(from, to) in edges {
            graph.add_edge((), node_indices[from], node_indices[to]);
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
        for &(from, to) in edges {
            graph.add_edge(node_indices[from], node_indices[to], ());
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
        for &(from, to) in edges {
            graph.add_edge(node_indices[from], node_indices[to], ());
        }
    }
    start.elapsed()
}

/// Benchmark gotgraph scoped traversal
pub fn benchmark_gotgraph_scoped_traversal(
    graph: &VecGraph<usize, ()>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        graph.scope(|ctx| {
            let mut total = 0;
            for node_tag in ctx.node_indices() {
                for _edge_tag in ctx.outgoing_edge_indices(node_tag) {
                    total += 1;
                }
            }
            total
        });
    }
    start.elapsed()
}

/// Benchmark gotgraph direct traversal
pub fn benchmark_gotgraph_direct_traversal(
    graph: &VecGraph<usize, ()>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut _total = 0;
        for node_idx in graph.node_indices() {
            for _edge_idx in graph.outgoing_edge_indices(node_idx) {
                _total += 1;
            }
        }
    }
    start.elapsed()
}

/// Benchmark petgraph DiGraph traversal
pub fn benchmark_petgraph_traversal(
    graph: &DiGraph<usize, ()>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut _total = 0;
        for node_idx in graph.node_indices() {
            for _edge_ref in graph.edges(node_idx) {
                _total += 1;
            }
        }
    }
    start.elapsed()
}

/// Benchmark petgraph StableDiGraph traversal
pub fn benchmark_petgraph_stable_traversal(
    graph: &StableDiGraph<usize, ()>,
    iterations: usize,
) -> std::time::Duration {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut _total = 0;
        for node_idx in graph.node_indices() {
            for _edge_ref in graph.edges(node_idx) {
                _total += 1;
            }
        }
    }
    start.elapsed()
}

/// Create test graphs for benchmarking
pub fn create_test_graphs(
    num_nodes: usize,
    edges: &[(usize, usize)],
) -> (VecGraph<usize, ()>, DiGraph<usize, ()>, StableDiGraph<usize, ()>) {
    // Create gotgraph
    let mut gotgraph_graph: VecGraph<usize, ()> = VecGraph::default();
    gotgraph_graph.scope_mut(|mut ctx| {
        let node_tags: Vec<_> = (0..num_nodes)
            .map(|i| ctx.add_node(i))
            .collect();
        for &(from, to) in edges {
            ctx.add_edge((), node_tags[from], node_tags[to]);
        }
    });

    // Create petgraph DiGraph
    let mut petgraph_graph = DiGraph::new();
    let petgraph_nodes: Vec<_> = (0..num_nodes)
        .map(|i| petgraph_graph.add_node(i))
        .collect();
    for &(from, to) in edges {
        petgraph_graph.add_edge(petgraph_nodes[from], petgraph_nodes[to], ());
    }

    // Create petgraph StableDiGraph
    let mut stable_graph = StableDiGraph::new();
    let stable_nodes: Vec<_> = (0..num_nodes)
        .map(|i| stable_graph.add_node(i))
        .collect();
    for &(from, to) in edges {
        stable_graph.add_edge(stable_nodes[from], stable_nodes[to], ());
    }

    (gotgraph_graph, petgraph_graph, stable_graph)
}

/// Run comprehensive benchmark for a given graph size
pub fn run_comprehensive_benchmark(size: usize) -> BenchmarkResult {
    let num_nodes = size;
    let num_edges = num_nodes * 2;
    
    let mut rng = StdRng::seed_from_u64(42);
    let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
    
    println!("Benchmarking size: {}", size);
    
    // Benchmark creation
    let gotgraph_scoped_time = benchmark_gotgraph_scoped_creation(num_nodes, &edges, 10);
    let gotgraph_direct_time = benchmark_gotgraph_direct_creation(num_nodes, &edges, 10);
    let petgraph_time = benchmark_petgraph_creation(num_nodes, &edges, 10);
    let petgraph_stable_time = benchmark_petgraph_stable_creation(num_nodes, &edges, 10);
    
    BenchmarkResult {
        graph_size: size,
        gotgraph_scoped_time_ns: gotgraph_scoped_time.as_nanos() as u64 / 10,
        gotgraph_direct_time_ns: gotgraph_direct_time.as_nanos() as u64 / 10,
        petgraph_time_ns: petgraph_time.as_nanos() as u64 / 10,
        petgraph_stable_time_ns: petgraph_stable_time.as_nanos() as u64 / 10,
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