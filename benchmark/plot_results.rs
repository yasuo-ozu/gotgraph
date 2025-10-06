use plotters::prelude::*;
use rand::prelude::*;
use std::time::Instant;

// Import gotgraph
use gotgraph::prelude::*;

// Import petgraph
use petgraph::graph::DiGraph;

#[derive(Debug, Clone)]
struct BenchmarkResult {
    graph_size: usize,
    gotgraph_time_ns: u64,
    petgraph_time_ns: u64,
}

fn generate_random_edges(num_nodes: usize, num_edges: usize, rng: &mut StdRng) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for _ in 0..num_edges {
        let from = rng.gen_range(0..num_nodes);
        let to = rng.gen_range(0..num_nodes);
        edges.push((from, to));
    }
    edges
}

fn benchmark_graph_creation() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    
    for &size in &[100, 200, 500, 1000, 2000, 5000] {
        println!("Benchmarking graph creation for size: {}", size);
        
        let num_nodes = size;
        let num_edges = num_nodes * 2;
        
        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
        
        // Benchmark GotGraph
        let gotgraph_start = Instant::now();
        for _ in 0..10 {
            let mut graph: VecGraph<usize, ()> = VecGraph::default();
            graph.scope_mut(|mut ctx| {
                let node_tags: Vec<_> = (0..num_nodes)
                    .map(|i| ctx.add_node(i))
                    .collect();
                for &(from, to) in &edges {
                    ctx.add_edge((), node_tags[from], node_tags[to]);
                }
            });
        }
        let gotgraph_time = gotgraph_start.elapsed();
        
        // Benchmark PetGraph
        let petgraph_start = Instant::now();
        for _ in 0..10 {
            let mut graph = DiGraph::new();
            let node_indices: Vec<_> = (0..num_nodes)
                .map(|i| graph.add_node(i))
                .collect();
            for &(from, to) in &edges {
                graph.add_edge(node_indices[from], node_indices[to], ());
            }
        }
        let petgraph_time = petgraph_start.elapsed();
        
        results.push(BenchmarkResult {
            graph_size: size,
            gotgraph_time_ns: gotgraph_time.as_nanos() as u64 / 10, // Average per iteration
            petgraph_time_ns: petgraph_time.as_nanos() as u64 / 10,
        });
    }
    
    results
}

fn benchmark_graph_traversal() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    
    for &size in &[100, 200, 500, 1000, 2000, 5000] {
        println!("Benchmarking graph traversal for size: {}", size);
        
        let num_nodes = size;
        let num_edges = num_nodes * 2;
        
        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
        
        // Create GotGraph
        let mut gotgraph_graph: VecGraph<usize, ()> = VecGraph::default();
        gotgraph_graph.scope_mut(|mut ctx| {
            let node_tags: Vec<_> = (0..num_nodes)
                .map(|i| ctx.add_node(i))
                .collect();
            for &(from, to) in &edges {
                ctx.add_edge((), node_tags[from], node_tags[to]);
            }
        });
        
        // Create PetGraph
        let mut petgraph_graph = DiGraph::new();
        let petgraph_nodes: Vec<_> = (0..num_nodes)
            .map(|i| petgraph_graph.add_node(i))
            .collect();
        for &(from, to) in &edges {
            petgraph_graph.add_edge(petgraph_nodes[from], petgraph_nodes[to], ());
        }
        
        // Benchmark GotGraph traversal
        let gotgraph_start = Instant::now();
        for _ in 0..100 {
            gotgraph_graph.scope(|ctx| {
                let mut total = 0;
                for node_tag in ctx.node_indices() {
                    for _edge_tag in ctx.outgoing_edge_indices(node_tag) {
                        total += 1;
                    }
                }
                total
            });
        }
        let gotgraph_time = gotgraph_start.elapsed();
        
        // Benchmark PetGraph traversal
        let petgraph_start = Instant::now();
        for _ in 0..100 {
            let mut _total = 0;
            for node_idx in petgraph_graph.node_indices() {
                for _edge_ref in petgraph_graph.edges(node_idx) {
                    _total += 1;
                }
            }
        }
        let petgraph_time = petgraph_start.elapsed();
        
        results.push(BenchmarkResult {
            graph_size: size,
            gotgraph_time_ns: gotgraph_time.as_nanos() as u64 / 100,
            petgraph_time_ns: petgraph_time.as_nanos() as u64 / 100,
        });
    }
    
    results
}

fn benchmark_memory_usage() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();
    
    for &size in &[500, 1000, 2000, 3000, 5000] {
        println!("Benchmarking memory usage for size: {}", size);
        
        let num_nodes = size;
        let num_edges = num_nodes * 2;
        
        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
        
        // Benchmark GotGraph memory usage (creating multiple graphs)
        let gotgraph_start = Instant::now();
        let mut gotgraph_graphs = Vec::new();
        for _ in 0..10 {
            let mut graph: VecGraph<usize, ()> = VecGraph::default();
            graph.scope_mut(|mut ctx| {
                let node_tags: Vec<_> = (0..num_nodes)
                    .map(|i| ctx.add_node(i))
                    .collect();
                for &(from, to) in &edges {
                    ctx.add_edge((), node_tags[from], node_tags[to]);
                }
            });
            gotgraph_graphs.push(graph);
        }
        let gotgraph_time = gotgraph_start.elapsed();
        drop(gotgraph_graphs);
        
        // Benchmark PetGraph memory usage
        let petgraph_start = Instant::now();
        let mut petgraph_graphs = Vec::new();
        for _ in 0..10 {
            let mut graph = DiGraph::new();
            let node_indices: Vec<_> = (0..num_nodes)
                .map(|i| graph.add_node(i))
                .collect();
            for &(from, to) in &edges {
                graph.add_edge(node_indices[from], node_indices[to], ());
            }
            petgraph_graphs.push(graph);
        }
        let petgraph_time = petgraph_start.elapsed();
        drop(petgraph_graphs);
        
        results.push(BenchmarkResult {
            graph_size: size,
            gotgraph_time_ns: gotgraph_time.as_nanos() as u64 / 10,
            petgraph_time_ns: petgraph_time.as_nanos() as u64 / 10,
        });
    }
    
    results
}

fn plot_results(results: &[BenchmarkResult], title: &str, filename: &str, y_label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let min_size = results.iter().map(|r| r.graph_size).min().unwrap_or(100) as f64;
    let max_size = results.iter().map(|r| r.graph_size).max().unwrap_or(1000) as f64;
    let min_time = results.iter()
        .map(|r| r.gotgraph_time_ns.min(r.petgraph_time_ns))
        .min()
        .unwrap_or(1000) as f64;
    let max_time = results.iter()
        .map(|r| r.gotgraph_time_ns.max(r.petgraph_time_ns))
        .max()
        .unwrap_or(1000) as f64;
    
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(
            (min_size * 0.8).log10()..(max_size * 1.2).log10(),
            (min_time * 0.8).log10()..(max_time * 1.2).log10()
        )?;
    
    chart.configure_mesh()
        .x_desc("Graph Size (nodes) - Log Scale")
        .y_desc(&format!("{} - Log Scale", y_label))
        .x_label_formatter(&|x| format!("{:.0}", 10_f64.powf(*x)))
        .y_label_formatter(&|y| format!("{:.0}", 10_f64.powf(*y)))
        .draw()?;
    
    // Plot GotGraph results
    chart
        .draw_series(LineSeries::new(
            results.iter().map(|r| ((r.graph_size as f64).log10(), (r.gotgraph_time_ns as f64).log10())),
            &RED,
        ))?
        .label("GotGraph (Scoped)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));
    
    // Plot PetGraph results
    chart
        .draw_series(LineSeries::new(
            results.iter().map(|r| ((r.graph_size as f64).log10(), (r.petgraph_time_ns as f64).log10())),
            &BLUE,
        ))?
        .label("PetGraph")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));
    
    // Add data points for GotGraph
    chart.draw_series(
        results.iter().map(|r| Circle::new(
            ((r.graph_size as f64).log10(), (r.gotgraph_time_ns as f64).log10()), 
            4, 
            RED.filled()
        ))
    )?;
    
    // Add data points for PetGraph
    chart.draw_series(
        results.iter().map(|r| Circle::new(
            ((r.graph_size as f64).log10(), (r.petgraph_time_ns as f64).log10()), 
            4, 
            BLUE.filled()
        ))
    )?;
    
    chart.configure_series_labels().draw()?;
    root.present()?;
    
    println!("Chart saved to: {}", filename);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running benchmarks and generating plots...");
    
    // Run benchmarks
    println!("\n=== Graph Creation Benchmark ===");
    let creation_results = benchmark_graph_creation();
    
    println!("\n=== Graph Traversal Benchmark ===");
    let traversal_results = benchmark_graph_traversal();
    
    println!("\n=== Memory Usage Benchmark ===");
    let memory_results = benchmark_memory_usage();
    
    // Generate plots
    println!("\n=== Generating Plots ===");
    
    plot_results(
        &creation_results,
        "Graph Creation Performance",
        "graph_creation_performance.svg",
        "Time (nanoseconds)"
    )?;
    
    plot_results(
        &traversal_results,
        "Graph Traversal Performance",
        "graph_traversal_performance.svg",
        "Time (nanoseconds)"
    )?;
    
    plot_results(
        &memory_results,
        "Memory Usage Performance",
        "memory_usage_performance.svg",
        "Time (nanoseconds)"
    )?;
    
    // Print summary
    println!("\n=== Results Summary ===");
    println!("Graph Creation:");
    for result in &creation_results {
        let ratio = result.gotgraph_time_ns as f64 / result.petgraph_time_ns as f64;
        println!("  Size {}: GotGraph {}ns, PetGraph {}ns (ratio: {:.2}x)",
                 result.graph_size, result.gotgraph_time_ns, result.petgraph_time_ns, ratio);
    }
    
    println!("\nGraph Traversal:");
    for result in &traversal_results {
        let ratio = result.gotgraph_time_ns as f64 / result.petgraph_time_ns as f64;
        println!("  Size {}: GotGraph {}ns, PetGraph {}ns (ratio: {:.2}x)",
                 result.graph_size, result.gotgraph_time_ns, result.petgraph_time_ns, ratio);
    }
    
    println!("\nMemory Usage:");
    for result in &memory_results {
        let ratio = result.gotgraph_time_ns as f64 / result.petgraph_time_ns as f64;
        println!("  Size {}: GotGraph {}ns, PetGraph {}ns (ratio: {:.2}x)",
                 result.graph_size, result.gotgraph_time_ns, result.petgraph_time_ns, ratio);
    }
    
    println!("\nPlots generated:");
    println!("- graph_creation_performance.svg");
    println!("- graph_traversal_performance.svg");
    println!("- memory_usage_performance.svg");
    
    Ok(())
}