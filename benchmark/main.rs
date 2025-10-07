use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::prelude::*;

// Import gotgraph
use gotgraph::prelude::*;
use gotgraph::algo::tarjan;

// Import petgraph
use petgraph::graph::DiGraph;
use petgraph::stable_graph::StableDiGraph;
use petgraph::algo::kosaraju_scc;

// Import our common benchmark library
use gotgraph_benchmark::{
    generate_random_edges,
    create_test_graphs,
    benchmark_gotgraph_scoped_creation,
    benchmark_gotgraph_direct_creation,
    benchmark_petgraph_creation,
    benchmark_petgraph_stable_creation,
    benchmark_gotgraph_scoped_traversal,
    benchmark_gotgraph_direct_traversal,
    benchmark_petgraph_traversal,
    benchmark_petgraph_stable_traversal,
};

fn bench_graph_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_creation");
    
    for size in [100, 500, 1000, 5000].iter() {
        let num_nodes = *size;
        let num_edges = num_nodes * 2; // 2 edges per node on average
        
        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
        
        group.bench_with_input(BenchmarkId::new("gotgraph", size), &(num_nodes, &edges), 
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let time = benchmark_gotgraph_scoped_creation(*num_nodes, edges, 1);
                    black_box(time)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let time = benchmark_petgraph_creation(*num_nodes, edges, 1);
                    black_box(time)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph_stable", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let time = benchmark_petgraph_stable_creation(*num_nodes, edges, 1);
                    black_box(time)
                })
            });
    }
    group.finish();
}

fn bench_graph_traversal(c: &mut Criterion) {
    let mut group = c.benchmark_group("graph_traversal");
    
    for size in [100, 500, 1000, 5000].iter() {
        let num_nodes = *size;
        let num_edges = num_nodes * 2;
        
        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
        
        let (gotgraph_graph, petgraph_graph, stable_graph) = create_test_graphs(num_nodes, &edges);
        
        group.bench_with_input(BenchmarkId::new("gotgraph", size), &gotgraph_graph,
            |b, graph| {
                b.iter(|| {
                    let time = benchmark_gotgraph_scoped_traversal(graph, 1);
                    black_box(time)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("gotgraph_direct", size), &gotgraph_graph,
            |b, graph| {
                b.iter(|| {
                    let time = benchmark_gotgraph_direct_traversal(graph, 1);
                    black_box(time)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph", size), &petgraph_graph,
            |b, graph| {
                b.iter(|| {
                    let time = benchmark_petgraph_traversal(graph, 1);
                    black_box(time)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph_stable", size), &stable_graph,
            |b, graph| {
                b.iter(|| {
                    let time = benchmark_petgraph_stable_traversal(graph, 1);
                    black_box(time)
                })
            });
    }
    group.finish();
}

fn bench_scc_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("strongly_connected_components");
    
    for size in [100, 500, 1000, 2000].iter() {
        let num_nodes = *size;
        let num_edges = num_nodes * 3; // More edges for interesting SCCs
        
        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
        
        let (gotgraph_graph, petgraph_graph, stable_scc_graph) = create_test_graphs(num_nodes, &edges);
        
        group.bench_with_input(BenchmarkId::new("gotgraph_tarjan", size), &gotgraph_graph,
            |b, graph| {
                b.iter(|| {
                    let components: Vec<_> = tarjan(graph).collect();
                    black_box(components)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph_kosaraju", size), &petgraph_graph,
            |b, graph| {
                b.iter(|| {
                    let components = kosaraju_scc(graph);
                    black_box(components)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph_stable_kosaraju", size), &stable_scc_graph,
            |b, graph| {
                b.iter(|| {
                    let components = kosaraju_scc(graph);
                    black_box(components)
                })
            });
    }
    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_efficiency");
    
    for size in [1000, 5000, 10000].iter() {
        let num_nodes = *size;
        let num_edges = num_nodes * 2;
        
        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
        
        group.bench_with_input(BenchmarkId::new("gotgraph_memory", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let time = benchmark_gotgraph_scoped_creation(*num_nodes, edges, 10);
                    black_box(time)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph_memory", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let time = benchmark_petgraph_creation(*num_nodes, edges, 10);
                    black_box(time)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph_stable_memory", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let time = benchmark_petgraph_stable_creation(*num_nodes, edges, 10);
                    black_box(time)
                })
            });
    }
    group.finish();
}

fn bench_scope_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("scope_operations");
    
    for size in [100, 500, 1000, 5000].iter() {
        let num_nodes = *size;
        let num_edges = num_nodes * 2;
        
        let mut rng = StdRng::seed_from_u64(42);
        let edges = generate_random_edges(num_nodes, num_edges, &mut rng);
        
        group.bench_with_input(BenchmarkId::new("gotgraph_scoped", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let mut graph: VecGraph<usize, ()> = VecGraph::default();
                    
                    graph.scope_mut(|mut ctx| {
                        // Add nodes
                        let node_tags: Vec<_> = (0..*num_nodes)
                            .map(|i| ctx.add_node(i))
                            .collect();
                        
                        // Add edges
                        for &(from, to) in edges.iter() {
                            ctx.add_edge((), node_tags[from], node_tags[to]);
                        }
                        
                        // Perform some operations within scope
                        let mut total = 0;
                        for node_tag in &node_tags {
                            for _edge_tag in ctx.outgoing_edge_indices(*node_tag) {
                                total += 1;
                            }
                        }
                        black_box(total)
                    });
                    
                    black_box(graph)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("gotgraph_direct", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let mut graph: VecGraph<usize, ()> = VecGraph::default();
                    
                    // Add nodes
                    let node_indices: Vec<_> = (0..*num_nodes)
                        .map(|i| graph.add_node(i))
                        .collect();
                    
                    // Add edges
                    for &(from, to) in edges.iter() {
                        graph.add_edge((), node_indices[from], node_indices[to]);
                    }
                    
                    // Perform some operations
                    let mut total = 0;
                    for &node_idx in &node_indices {
                        for _edge_idx in graph.outgoing_edge_indices(node_idx) {
                            total += 1;
                        }
                    }
                    black_box(total);
                    
                    black_box(graph)
                })
            });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_graph_creation,
    bench_graph_traversal,
    bench_scc_algorithms,
    bench_memory_usage,
    bench_scope_operations
);
criterion_main!(benches);