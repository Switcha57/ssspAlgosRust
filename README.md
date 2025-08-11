# SSSP Algorithms Comparison

A simple Rust project comparing the classic Dijkstra's algorithm with a new SSSP (Single Source Shortest Path) algorithm.

## Structure

```
src/
├── lib.rs              # Main library exports
├── algorithms/         # Algorithm implementations
│   ├── mod.rs
│   ├── dijkstra.rs     # Classic Dijkstra implementation
│   └── new_algorithm.rs # New SSSP algorithm (replace with your implementation)
├── graph.rs            # Graph data structure
├── utils.rs            # Utilities for testing and benchmarking
└── bin/
    ├── demo.rs         # Simple demo comparing both algorithms
    └── benchmark.rs    # Performance benchmark

benches/
└── sssp_comparison.rs  # Criterion-based benchmarks
```

## Usage

### Run the demo
```bash
cargo run --bin demo
```

### Run performance benchmarks
```bash
cargo run --bin benchmark
```

### Run detailed benchmarks with Criterion
```bash
cargo bench
```

### Run tests
```bash
cargo test
```

## Implementation Notes

- Replace the placeholder implementation in `src/algorithms/new_algorithm.rs` with your actual new algorithm
- The current "new algorithm" is just a placeholder that demonstrates the project structure
- Both algorithms should return the same results for correctness verification
- Benchmarking compares performance between the two implementations

## Dependencies

- `criterion`: For detailed benchmarking with statistical analysis
- `rand`: For generating random test graphs
- `serde`: For serializing benchmark results
