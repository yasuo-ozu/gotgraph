use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rand::prelude::*;

// Import gotgraph
use gotgraph::prelude::*;
use gotgraph::algo::tarjan;

// Import petgraph
use petgraph::graph::DiGraph;
use petgraph::algo::kosaraju_scc;

fn generate_random_edges(num_nodes: usize, num_edges: usize, rng: &mut StdRng) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for _ in 0..num_edges {
        let from = rng.gen_range(0..num_nodes);
        let to = rng.gen_range(0..num_nodes);
        edges.push((from, to));
    }
    edges
}

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
                    });
                    
                    black_box(graph)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let mut graph = DiGraph::new();
                    
                    // Add nodes
                    let node_indices: Vec<_> = (0..*num_nodes)
                        .map(|i| graph.add_node(i))
                        .collect();
                    
                    // Add edges
                    for &(from, to) in edges.iter() {
                        graph.add_edge(node_indices[from], node_indices[to], ());
                    }
                    
                    black_box(graph)
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
        
        // Create gotgraph
        let mut gotgraph_graph: VecGraph<usize, ()> = VecGraph::default();
        gotgraph_graph.scope_mut(|mut ctx| {
            let node_tags: Vec<_> = (0..num_nodes)
                .map(|i| ctx.add_node(i))
                .collect();
            for &(from, to) in &edges {
                ctx.add_edge((), node_tags[from], node_tags[to]);
            }
        });
        
        // Create petgraph
        let mut petgraph_graph = DiGraph::new();
        let petgraph_nodes: Vec<_> = (0..num_nodes)
            .map(|i| petgraph_graph.add_node(i))
            .collect();
        for &(from, to) in &edges {
            petgraph_graph.add_edge(petgraph_nodes[from], petgraph_nodes[to], ());
        }
        
        group.bench_with_input(BenchmarkId::new("gotgraph", size), &gotgraph_graph,
            |b, graph| {
                b.iter(|| {
                    graph.scope(|ctx| {
                        let mut total = 0;
                        for node_tag in ctx.node_indices() {
                            for _edge_tag in ctx.outgoing_edge_indices(node_tag) {
                                total += 1;
                            }
                        }
                        black_box(total)
                    })
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph", size), &petgraph_graph,
            |b, graph| {
                b.iter(|| {
                    let mut total = 0;
                    for node_idx in graph.node_indices() {
                        for _edge_ref in graph.edges(node_idx) {
                            total += 1;
                        }
                    }
                    black_box(total)
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
        
        // Create gotgraph
        let mut gotgraph_graph: VecGraph<usize, ()> = VecGraph::default();
        gotgraph_graph.scope_mut(|mut ctx| {
            let node_tags: Vec<_> = (0..num_nodes)
                .map(|i| ctx.add_node(i))
                .collect();
            for &(from, to) in &edges {
                ctx.add_edge((), node_tags[from], node_tags[to]);
            }
        });
        
        // Create petgraph
        let mut petgraph_graph = DiGraph::new();
        let petgraph_nodes: Vec<_> = (0..num_nodes)
            .map(|i| petgraph_graph.add_node(i))
            .collect();
        for &(from, to) in &edges {
            petgraph_graph.add_edge(petgraph_nodes[from], petgraph_nodes[to], ());
        }
        
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
                    let mut graphs = Vec::new();
                    for _ in 0..10 {
                        let mut graph: VecGraph<usize, ()> = VecGraph::default();
                        graph.scope_mut(|mut ctx| {
                            let node_tags: Vec<_> = (0..*num_nodes)
                                .map(|i| ctx.add_node(i))
                                .collect();
                            for &(from, to) in edges.iter() {
                                ctx.add_edge((), node_tags[from], node_tags[to]);
                            }
                        });
                        graphs.push(graph);
                    }
                    black_box(graphs)
                })
            });
        
        group.bench_with_input(BenchmarkId::new("petgraph_memory", size), &(num_nodes, &edges),
            |b, (num_nodes, edges)| {
                b.iter(|| {
                    let mut graphs = Vec::new();
                    for _ in 0..10 {
                        let mut graph = DiGraph::new();
                        let node_indices: Vec<_> = (0..*num_nodes)
                            .map(|i| graph.add_node(i))
                            .collect();
                        for &(from, to) in edges.iter() {
                            graph.add_edge(node_indices[from], node_indices[to], ());
                        }
                        graphs.push(graph);
                    }
                    black_box(graphs)
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