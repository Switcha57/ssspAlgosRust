use std::collections::HashMap;


pub fn bfs(graph: &crate::graph::Graph, start: usize) -> HashMap<usize, usize> {
 
    let mut distances = HashMap::new();
    
    // Initialize distances
    for &node in graph.nodes() {
        distances.insert(node, usize::MAX);
    }
    distances.insert(start, 0);
    
    
    let mut queue = vec![start];
    let mut visited = HashMap::new();
    
    while let Some(current) = queue.pop() {
        if visited.contains_key(&current) {
            continue;
        }
        visited.insert(current, true);
        
        for &(neighbor, weight) in graph.neighbors(current) {
            let new_distance = distances[&current].saturating_add(weight);
            if new_distance < distances[&neighbor] {
                distances.insert(neighbor, new_distance);
                queue.push(neighbor);
            }
        }
    }
    
    distances
}
