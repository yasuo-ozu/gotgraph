# GotGraph vs PetGraph Benchmarks

This directory contains comprehensive benchmarks comparing `gotgraph` with `petgraph`, one of the most popular graph libraries in Rust.

## Running the Benchmarks

### Standard Criterion Benchmarks
```bash
cargo bench
```

For a quick test run:
```bash
cargo bench -- --quick
```

### Generating Performance Plots
To generate SVG plots showing performance comparison:
```bash
cargo run --bin plot_results
```

This will create three SVG files:
- `graph_creation_performance.svg` - Graph creation performance vs graph size
- `graph_traversal_performance.svg` - Graph traversal performance vs graph size  
- `memory_usage_performance.svg` - Memory allocation performance vs graph size

## Benchmark Categories

### 1. Graph Creation
Compares the performance of creating graphs with nodes and edges.
- **GotGraph**: Uses scoped operations (`scope_mut`)
- **PetGraph**: Uses direct operations

### 2. Graph Traversal
Measures the speed of iterating over outgoing edges for all nodes.
- **GotGraph**: Uses scoped operations (`scope`)
- **PetGraph**: Uses direct operations

### 3. Strongly Connected Components
Compares algorithm implementations:
- **GotGraph**: Tarjan's algorithm (graphs created with scoped operations)
- **PetGraph**: Kosaraju's algorithm

### 4. Memory Efficiency
Tests memory allocation patterns when creating multiple graphs.
- **GotGraph**: Uses scoped operations for graph creation
- **PetGraph**: Uses direct operations

### 5. Scope Operations
GotGraph-specific test comparing scoped vs direct operations on the same library.

## Results Summary

Based on the benchmark runs using scoped operations for GotGraph, here are the key findings:

### Graph Creation (Using Scoped Operations)
- **Performance ratio**: GotGraph is 1.15x to 1.60x slower than PetGraph
- **Trend**: Performance gap narrows with larger graphs (1.24x slower at 5000 nodes)
- **Trade-off**: Acceptable overhead for the safety guarantees provided

### Graph Traversal (Using Scoped Operations)
- **Performance ratio**: GotGraph is actually **faster** than PetGraph (0.86x to 0.94x)
- **Consistent advantage**: GotGraph shows better traversal performance across all graph sizes
- **Scoped benefit**: Demonstrates the efficiency of GotGraph's scoped operations

### Memory Efficiency (Using Scoped Operations)  
- **Performance ratio**: GotGraph is 1.29x to 1.34x slower for memory operations
- **Consistent overhead**: ~30% overhead for memory allocation patterns
- **Predictable**: Overhead remains relatively constant across different graph sizes

### Strongly Connected Components
- **PetGraph (Kosaraju)**: Still significantly faster (5-10x) due to algorithm differences
- **GotGraph (Tarjan)**: More comprehensive algorithm but slower execution

### Key Insights from Plots
- **Graph Creation**: Linear performance scaling for both libraries
- **Graph Traversal**: GotGraph shows superior performance, especially for larger graphs
- **Memory Usage**: Both libraries scale linearly, with consistent overhead ratio

### Scope Operations (GotGraph Internal Comparison)
- **Scoped operations**: Generally faster than direct operations
- **Zero-cost abstractions**: Confirmed - scopes often improve performance
- **Best practice**: Use scoped operations for optimal GotGraph performance

## Trade-offs

### GotGraph Advantages
- **Compile-time safety**: Prevents many graph-related bugs
- **Type safety**: Strong typing prevents index misuse
- **Zero-cost abstractions**: Safety features have minimal runtime cost
- **Scoped operations**: Prevent use-after-free bugs

### PetGraph Advantages
- **Raw performance**: Faster for most operations
- **Memory efficiency**: Lower memory usage
- **Mature algorithms**: Highly optimized implementations
- **Broader algorithm support**: More graph algorithms available

## Conclusion

GotGraph prioritizes **safety and correctness** over raw performance, while PetGraph focuses on **speed and efficiency**. The performance difference is generally acceptable for most applications, especially considering the compile-time safety benefits GotGraph provides.

Choose GotGraph when:
- Safety and correctness are paramount
- You want to prevent graph-related bugs at compile time
- Performance differences are acceptable for your use case

Choose PetGraph when:
- Maximum performance is critical
- You need extensive algorithm support
- Memory usage is a primary concern