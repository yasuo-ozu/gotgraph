# GotGraph vs PetGraph Benchmarks

This directory contains comprehensive benchmarks comparing `gotgraph` with `petgraph`, one of the most popular graph libraries in Rust.

## Running the Benchmarks

```bash
cargo bench
```

For a quick test run:
```bash
cargo bench -- --quick
```

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
- **Small graphs (100 nodes)**: GotGraph competitive, sometimes faster
- **Medium graphs (500-1000 nodes)**: PetGraph faster (~20-25%)
- **Large graphs (5000 nodes)**: PetGraph faster (~25-30%)

### Graph Traversal (Using Scoped Operations)
- **Small graphs**: Very competitive performance
- **Medium graphs**: Similar performance
- **Large graphs**: GotGraph surprisingly competitive with scoped operations

### Strongly Connected Components
- **PetGraph (Kosaraju)**: Still significantly faster (5-10x)
- **GotGraph (Tarjan)**: Using scoped graph creation, consistent performance

### Memory Efficiency (Using Scoped Operations)  
- **PetGraph**: More memory efficient across all tested sizes
- **GotGraph**: Better memory performance with scoped operations than direct operations

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