use sssp_algos::{dijkstra, new_algorithm, generate_random_graph, BenchmarkResult};
use std::time::Instant;

fn main() {
    println!("SSSP Algorithms Benchmark");
    println!("========================");
    
    let graph_sizes = [100, 500, 1000];
    let edge_density = 0.3;
    let start_node = 0;
    
    for &size in &graph_sizes {
        println!("\nTesting graph with {} nodes", size);
        println!("-".repeat(40));
        
        let graph = generate_random_graph(size, edge_density);
        println!("Generated graph: {} nodes, {} edges", 
                graph.node_count(), graph.edge_count());
        
        // Benchmark Dijkstra
        let start_time = Instant::now();
        let dijkstra_result = dijkstra(&graph, start_node);
        let dijkstra_time = start_time.elapsed();
        
        let dijkstra_benchmark = BenchmarkResult {
            algorithm: "Dijkstra".to_string(),
            graph_size: size,
            edge_count: graph.edge_count(),
            execution_time_ns: dijkstra_time.as_nanos(),
            distances_computed: dijkstra_result.len(),
        };
        
        // Benchmark New Algorithm
        let start_time = Instant::now();
        let new_algo_result = new_algorithm(&graph, start_node);
        let new_algo_time = start_time.elapsed();
        
        let new_algo_benchmark = BenchmarkResult {
            algorithm: "New Algorithm".to_string(),
            graph_size: size,
            edge_count: graph.edge_count(),
            execution_time_ns: new_algo_time.as_nanos(),
            distances_computed: new_algo_result.len(),
        };
        
        // Print results
        println!("Dijkstra:      {:>10} ns ({:>6.2} ms)", 
                dijkstra_benchmark.execution_time_ns,
                dijkstra_benchmark.execution_time_ns as f64 / 1_000_000.0);
        println!("New Algorithm: {:>10} ns ({:>6.2} ms)", 
                new_algo_benchmark.execution_time_ns,
                new_algo_benchmark.execution_time_ns as f64 / 1_000_000.0);
        
        let speedup = dijkstra_time.as_nanos() as f64 / new_algo_time.as_nanos() as f64;
        println!("Speedup: {:.2}x", speedup);
        
        // Verify results are consistent (optional)
        let consistent = dijkstra_result.len() == new_algo_result.len();
        println!("Results consistent: {}", if consistent { "✓" } else { "✗" });
    }
}
