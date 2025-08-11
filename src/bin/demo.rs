use sssp_algos::{dijkstra, new_algorithm, create_test_graph};

fn main() {
    println!("SSSP Algorithms Demo");
    println!("==================");
    
    let graph = create_test_graph();
    let start_node = 0;
    
    println!("Test graph: {} nodes, {} edges", 
            graph.node_count(), graph.edge_count());
    println!("Starting from node: {}", start_node);
    println!();
    
    // Run Dijkstra
    println!("Dijkstra's Algorithm Results:");
    let dijkstra_distances = dijkstra(&graph, start_node);
    for (node, distance) in &dijkstra_distances {
        if *distance == usize::MAX {
            println!("  Node {}: unreachable", node);
        } else {
            println!("  Node {}: distance {}", node, distance);
        }
    }
    
    println!();
    
    // Run new algorithm
    println!("New Algorithm Results:");
    let new_algo_distances = new_algorithm(&graph, start_node);
    for (node, distance) in &new_algo_distances {
        if *distance == usize::MAX {
            println!("  Node {}: unreachable", node);
        } else {
            println!("  Node {}: distance {}", node, distance);
        }
    }
    
    println!();
    
    // Compare results
    println!("Comparison:");
    let mut differences = 0;
    for node in graph.nodes() {
        let dijkstra_dist = dijkstra_distances.get(node).unwrap_or(&usize::MAX);
        let new_algo_dist = new_algo_distances.get(node).unwrap_or(&usize::MAX);
        
        if dijkstra_dist != new_algo_dist {
            println!("  Node {}: Dijkstra={}, NewAlgo={} ✗", 
                    node, dijkstra_dist, new_algo_dist);
            differences += 1;
        } else {
            println!("  Node {}: {} ✓", node, dijkstra_dist);
        }
    }
    
    if differences == 0 {
        println!("\n✓ All results match!");
    } else {
        println!("\n✗ Found {} differences", differences);
    }
}
