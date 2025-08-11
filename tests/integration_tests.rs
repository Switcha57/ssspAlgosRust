use sssp_algos::{dijkstra, bfs, create_test_graph, generate_random_graph};

#[test]
fn test_algorithms_consistency() {
    let graph = create_test_graph();
    let start_node = 0;
    
    let dijkstra_result = dijkstra(&graph, start_node);
    let new_algo_result = bfs(&graph, start_node);
    
    // Both algorithms should compute distances for the same nodes
    assert_eq!(dijkstra_result.len(), new_algo_result.len());
    
    // For a correct SSSP algorithm, results should match Dijkstra
    // Note: This test might fail if your new algorithm is not yet implemented correctly
    for node in graph.nodes() {
        let dijkstra_dist = dijkstra_result.get(node).unwrap_or(&usize::MAX);
        let new_algo_dist = new_algo_result.get(node).unwrap_or(&usize::MAX);
        
        // For now, just ensure both algorithms return some result
        assert!(dijkstra_result.contains_key(node));
        assert!(new_algo_result.contains_key(node));
        
        // Uncomment this when your new algorithm is correctly implemented:
        assert_eq!(dijkstra_dist, new_algo_dist, "Distance mismatch for node {}", node);
    }
}

#[test]
fn test_dijkstra_correctness() {
    let graph = create_test_graph();
    let start_node = 0;
    
    let result = dijkstra(&graph, start_node);
    
    // Verify some known shortest distances in the test graph
    assert_eq!(result[&0], 0);  // Distance to self is 0
    assert_eq!(result[&1], 4);  // 0 -> 1 (weight 4)
    assert_eq!(result[&4], 2);  // 0 -> 4 (weight 2)
    assert_eq!(result[&5], 3);  // 0 -> 4 -> 5 (weight 2 + 1 = 3)
}

#[test]
fn test_random_graph_generation() {
    let graph = generate_random_graph(10, 0.3);
    
    assert_eq!(graph.node_count(), 10);
    assert!(graph.edge_count() > 0);
    
    // Should be able to run algorithms on generated graph
    let dijkstra_result = dijkstra(&graph, 0);
    let new_algo_result = bfs(&graph, 0);
    
    assert_eq!(dijkstra_result.len(), 10);
    assert_eq!(new_algo_result.len(), 10);
}
