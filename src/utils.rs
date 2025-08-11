use crate::graph::Graph;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub algorithm: String,
    pub graph_size: usize,
    pub edge_count: usize,
    pub execution_time_ns: u128,
    pub distances_computed: usize,
}

/// Generate a random connected graph for testing
pub fn generate_random_graph(nodes: usize, edge_density: f64) -> Graph {
    let mut graph = Graph::new();
    let mut rng = rand::thread_rng();
    
    // Add all nodes
    for i in 0..nodes {
        graph.add_node(i);
    }
    
    // Ensure the graph is connected by creating a spanning tree
    for i in 1..nodes {
        let parent = rng.gen_range(0..i);
        let weight = rng.gen_range(1..=100);
        graph.add_bidirectional_edge(parent, i, weight);
    }
    
    // Add additional edges based on density
    let max_edges = nodes * (nodes - 1) / 2;
    let target_edges = (max_edges as f64 * edge_density) as usize;
    let current_edges = nodes - 1; // edges from spanning tree
    
    for _ in 0..(target_edges.saturating_sub(current_edges)) {
        let from = rng.gen_range(0..nodes);
        let to = rng.gen_range(0..nodes);
        
        if from != to {
            let weight = rng.gen_range(1..=100);
            graph.add_edge(from, to, weight);
        }
    }
    
    graph
}

/// Create a simple test graph for verification
pub fn create_test_graph() -> Graph {
    let mut graph = Graph::new();
    
    // Create a simple graph: 0 -> 1 -> 2 -> 3
    //                       |    |    |
    //                       v    v    v
    //                       4 -> 5 -> 6
    
    graph.add_edge(0, 1, 4);
    graph.add_edge(0, 4, 2);
    graph.add_edge(1, 2, 1);
    graph.add_edge(1, 5, 5);
    graph.add_edge(2, 3, 3);
    graph.add_edge(2, 6, 2);
    graph.add_edge(4, 5, 1);
    graph.add_edge(5, 6, 3);
    
    graph
}
