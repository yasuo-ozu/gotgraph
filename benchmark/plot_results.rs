use plotters::prelude::*;
use rand::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

// Import gotgraph
use gotgraph::prelude::*;

// Import petgraph
use petgraph::graph::DiGraph;
use petgraph::stable_graph::StableDiGraph;

// Import graphlib
use graphlib::Graph as GraphlibGraph;

// Import our common benchmark library
use gotgraph_benchmark::{
    benchmark_gotgraph_direct_traversal, benchmark_gotgraph_scoped_traversal,
    benchmark_graphlib_traversal, benchmark_pathfinding_traversal,
    benchmark_petgraph_stable_traversal, benchmark_petgraph_traversal, create_test_graphs,
    create_test_graphs_with_indices, generate_random_edges, print_performance_summary,
    run_comprehensive_benchmark, BenchmarkResult,
};

fn benchmark_graph_creation() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    for &size in &[100, 200, 500, 1000, 2000, 5000, 10000] {
        results.push(run_comprehensive_benchmark(size, 1000));
    }

    results
}

fn benchmark_graph_traversal() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    for &size in &[100, 200, 500, 1000, 2000, 5000, 10000] {
        println!("Benchmarking graph traversal for size: {}", size);

        let num_nodes = size;
        let num_edges = num_nodes * 2;

        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);

        let (gotgraph_graph, petgraph_graph, stable_graph, adjacency_list, graphlib_graph) =
            create_test_graphs(num_nodes, &edges);

        // Benchmark traversals - use more iterations for accurate measurement
        let iterations = 10000; // Increased from 100 to get more accurate measurements
        let gotgraph_scoped_time = benchmark_gotgraph_scoped_traversal(&gotgraph_graph, iterations);
        let gotgraph_direct_time = benchmark_gotgraph_direct_traversal(&gotgraph_graph, iterations);
        let petgraph_time = benchmark_petgraph_traversal(&petgraph_graph, iterations);
        let stable_time = benchmark_petgraph_stable_traversal(&stable_graph, iterations);
        let pathfinding_time = benchmark_pathfinding_traversal(&adjacency_list, iterations);
        let graphlib_time = benchmark_graphlib_traversal(&graphlib_graph, iterations);

        results.push(BenchmarkResult {
            graph_size: size,
            gotgraph_scoped_time_ns: gotgraph_scoped_time.as_nanos() as u64 / iterations as u64,
            gotgraph_direct_time_ns: gotgraph_direct_time.as_nanos() as u64 / iterations as u64,
            petgraph_time_ns: petgraph_time.as_nanos() as u64 / iterations as u64,
            petgraph_stable_time_ns: stable_time.as_nanos() as u64 / iterations as u64,
            pathfinding_time_ns: pathfinding_time.as_nanos() as u64 / iterations as u64,
            graphlib_time_ns: graphlib_time.as_nanos() as u64 / iterations as u64,
        });
    }

    results
}

fn benchmark_memory_usage() -> Vec<BenchmarkResult> {
    let mut results = Vec::new();

    for &size in &[500, 1000, 2000, 3000, 5000, 10000, 20000, 50000] {
        println!("Benchmarking memory usage for size: {}", size);

        let num_nodes = size;
        let num_edges = num_nodes * 2;

        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);

        // Benchmark GotGraph memory usage (creating multiple graphs)
        let gotgraph_start = Instant::now();
        let mut gotgraph_graphs = Vec::new();
        for _ in 0..10 {
            let mut graph: VecGraph<usize, usize> = VecGraph::default();
            graph.scope_mut(|mut ctx| {
                let node_tags: Vec<_> = (0..num_nodes).map(|i| ctx.add_node(i)).collect();
                for (edge_idx, &(from, to)) in edges.iter().enumerate() {
                    ctx.add_edge(edge_idx, node_tags[from], node_tags[to]);
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
            let node_indices: Vec<_> = (0..num_nodes).map(|i| graph.add_node(i)).collect();
            for (edge_idx, &(from, to)) in edges.iter().enumerate() {
                graph.add_edge(node_indices[from], node_indices[to], edge_idx);
            }
            petgraph_graphs.push(graph);
        }
        let petgraph_time = petgraph_start.elapsed();
        drop(petgraph_graphs);

        // Benchmark PetGraph Stable memory usage
        let stable_start = Instant::now();
        let mut stable_graphs = Vec::new();
        for _ in 0..10 {
            let mut graph = StableDiGraph::new();
            let node_indices: Vec<_> = (0..num_nodes).map(|i| graph.add_node(i)).collect();
            for (edge_idx, &(from, to)) in edges.iter().enumerate() {
                graph.add_edge(node_indices[from], node_indices[to], edge_idx);
            }
            stable_graphs.push(graph);
        }
        let stable_time = stable_start.elapsed();
        drop(stable_graphs);

        // Benchmark Pathfinding memory usage
        let pathfinding_start = Instant::now();
        let mut pathfinding_graphs = Vec::new();
        for _ in 0..10 {
            let mut adjacency_list: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
            for i in 0..num_nodes {
                adjacency_list.insert(i, Vec::new());
            }
            for (edge_idx, &(from, to)) in edges.iter().enumerate() {
                adjacency_list.get_mut(&from).unwrap().push((to, edge_idx));
            }
            pathfinding_graphs.push(adjacency_list);
        }
        let pathfinding_time = pathfinding_start.elapsed();
        drop(pathfinding_graphs);

        // Benchmark GraphLib memory usage
        let graphlib_start = Instant::now();
        let mut graphlib_graphs = Vec::new();
        for _ in 0..10 {
            let mut graph = GraphlibGraph::new();
            let mut vertex_ids = Vec::new();
            for i in 0..num_nodes {
                let id = graph.add_vertex(i);
                vertex_ids.push(id);
            }
            for &(from, to) in edges.iter() {
                graph.add_edge(&vertex_ids[from], &vertex_ids[to]).ok(); // Ignore errors for DAG constraints
            }
            graphlib_graphs.push(graph);
        }
        let graphlib_time = graphlib_start.elapsed();
        drop(graphlib_graphs);

        results.push(BenchmarkResult {
            graph_size: size,
            gotgraph_scoped_time_ns: gotgraph_time.as_nanos() as u64 / 10,
            gotgraph_direct_time_ns: gotgraph_time.as_nanos() as u64 / 10,
            petgraph_time_ns: petgraph_time.as_nanos() as u64 / 10,
            petgraph_stable_time_ns: stable_time.as_nanos() as u64 / 10,
            pathfinding_time_ns: pathfinding_time.as_nanos() as u64 / 10,
            graphlib_time_ns: graphlib_time.as_nanos() as u64 / 10,
        });
    }

    results
}

fn plot_results(
    results: &[BenchmarkResult],
    title: &str,
    filename: &str,
    y_label: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = SVGBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_size = results.iter().map(|r| r.graph_size).min().unwrap_or(100) as f64;
    let max_size = results.iter().map(|r| r.graph_size).max().unwrap_or(1000) as f64;
    let min_time = results
        .iter()
        .map(|r| {
            r.gotgraph_scoped_time_ns.min(
                r.gotgraph_direct_time_ns
                    .min(r.petgraph_time_ns.min(r.petgraph_stable_time_ns)),
            )
        })
        .min()
        .unwrap_or(1000) as f64;
    let max_time = results
        .iter()
        .map(|r| {
            r.gotgraph_scoped_time_ns.max(
                r.gotgraph_direct_time_ns
                    .max(r.petgraph_time_ns.max(r.petgraph_stable_time_ns)),
            )
        })
        .max()
        .unwrap_or(1000) as f64;

    // Protect against log(0) or log(negative) which can cause infinite loops
    let min_time = min_time.max(1.0);
    let max_time = max_time.max(1.0);

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(50)
        .y_label_area_size(80)
        .build_cartesian_2d(
            (min_size * 0.8).log10()..(max_size * 1.2).log10(),
            (min_time * 0.8).log10()..(max_time * 1.2).log10(),
        )?;

    chart
        .configure_mesh()
        .x_desc("Graph Size (nodes) - Log Scale")
        .y_desc(&format!("{} - Log Scale", y_label))
        .x_label_formatter(&|x| format!("{:.0}", 10_f64.powf(*x)))
        .y_label_formatter(&|y| format!("{:.0}", 10_f64.powf(*y)))
        .draw()?;

    // Plot GotGraph Direct results
    chart
        .draw_series(LineSeries::new(
            results.iter().map(|r| {
                (
                    (r.graph_size as f64).log10(),
                    (r.gotgraph_direct_time_ns as f64).log10(),
                )
            }),
            &RGBColor(255, 100, 100),
        ))?
        .label("GotGraph (Direct)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RGBColor(255, 100, 100)));

    // Plot PetGraph results
    chart
        .draw_series(LineSeries::new(
            results.iter().map(|r| {
                (
                    (r.graph_size as f64).log10(),
                    (r.petgraph_time_ns as f64).log10(),
                )
            }),
            &BLUE,
        ))?
        .label("PetGraph (DiGraph)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));

    // Plot PetGraph StableGraph results
    chart
        .draw_series(LineSeries::new(
            results.iter().map(|r| {
                (
                    (r.graph_size as f64).log10(),
                    (r.petgraph_stable_time_ns as f64).log10(),
                )
            }),
            &GREEN,
        ))?
        .label("PetGraph (StableGraph)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &GREEN));

    // Plot GotGraph Scoped results (drawn last to appear in front)
    chart
        .draw_series(LineSeries::new(
            results.iter().map(|r| {
                (
                    (r.graph_size as f64).log10(),
                    (r.gotgraph_scoped_time_ns as f64).log10(),
                )
            }),
            &RGBColor(255, 165, 0), // Orange color
        ))?
        .label("GotGraph (Scoped)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RGBColor(255, 165, 0)));

    // Plot Pathfinding results
    chart
        .draw_series(LineSeries::new(
            results.iter().map(|r| {
                (
                    (r.graph_size as f64).log10(),
                    (r.pathfinding_time_ns as f64).log10(),
                )
            }),
            &RGBColor(255, 0, 255), // Magenta color
        ))?
        .label("Pathfinding")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RGBColor(255, 0, 255)));

    // Plot GraphLib results
    chart
        .draw_series(LineSeries::new(
            results.iter().map(|r| {
                (
                    (r.graph_size as f64).log10(),
                    (r.graphlib_time_ns as f64).log10(),
                )
            }),
            &RGBColor(0, 255, 255), // Cyan color
        ))?
        .label("GraphLib")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RGBColor(0, 255, 255)));

    // Add data points for all implementations
    chart.draw_series(results.iter().map(|r| {
        Circle::new(
            (
                (r.graph_size as f64).log10(),
                (r.gotgraph_direct_time_ns as f64).log10(),
            ),
            4,
            RGBColor(255, 100, 100).filled(),
        )
    }))?;

    chart.draw_series(results.iter().map(|r| {
        Circle::new(
            (
                (r.graph_size as f64).log10(),
                (r.petgraph_time_ns as f64).log10(),
            ),
            4,
            BLUE.filled(),
        )
    }))?;

    chart.draw_series(results.iter().map(|r| {
        Circle::new(
            (
                (r.graph_size as f64).log10(),
                (r.petgraph_stable_time_ns as f64).log10(),
            ),
            4,
            GREEN.filled(),
        )
    }))?;

    chart.draw_series(results.iter().map(|r| {
        Circle::new(
            (
                (r.graph_size as f64).log10(),
                (r.pathfinding_time_ns as f64).log10(),
            ),
            4,
            RGBColor(255, 0, 255).filled(), // Magenta color to match line
        )
    }))?;

    chart.draw_series(results.iter().map(|r| {
        Circle::new(
            (
                (r.graph_size as f64).log10(),
                (r.graphlib_time_ns as f64).log10(),
            ),
            4,
            RGBColor(0, 255, 255).filled(), // Cyan color to match line
        )
    }))?;

    // Draw GotGraph Scoped points last so they appear in front
    chart.draw_series(results.iter().map(|r| {
        Circle::new(
            (
                (r.graph_size as f64).log10(),
                (r.gotgraph_scoped_time_ns as f64).log10(),
            ),
            4,
            RGBColor(255, 165, 0).filled(), // Orange color to match line
        )
    }))?;

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
        "Time (nanoseconds)",
    )?;

    plot_results(
        &traversal_results,
        "Graph Traversal Performance",
        "graph_traversal_performance.svg",
        "Time (nanoseconds)",
    )?;

    plot_results(
        &memory_results,
        "Memory Usage Performance",
        "memory_usage_performance.svg",
        "Time (nanoseconds)",
    )?;

    // Print summaries
    println!("\n=== Results Summary ===");
    print_performance_summary(&creation_results, "Graph Creation");
    print_performance_summary(&traversal_results, "Graph Traversal");
    print_performance_summary(&memory_results, "Memory Usage");

    println!("\nPlots generated:");
    println!("- graph_creation_performance.svg");
    println!("- graph_traversal_performance.svg");
    println!("- memory_usage_performance.svg");

    Ok(())
}
