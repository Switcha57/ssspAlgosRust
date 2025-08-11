use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Classic Dijkstra's algorithm implementation
pub fn dijkstra(graph: &crate::graph::Graph, start: usize) -> HashMap<usize, usize> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    // Initialize distances
    for &node in graph.nodes() {
        distances.insert(node, usize::MAX);
    }
    distances.insert(start, 0);
    
    heap.push(State { cost: 0, position: start });
    
    while let Some(State { cost, position }) = heap.pop() {
        if cost > distances[&position] {
            continue;
        }
        
        for &(neighbor, weight) in graph.neighbors(position) {
            let next_cost = cost + weight;
            
            if next_cost < distances[&neighbor] {
                distances.insert(neighbor, next_cost);
                heap.push(State { cost: next_cost, position: neighbor });
            }
        }
    }
    
    distances
}
