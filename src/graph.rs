use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Graph {
    adjacency_list: HashMap<usize, Vec<(usize, usize)>>,
    nodes: Vec<usize>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
            nodes: Vec::new(),
        }
    }
    
    pub fn add_node(&mut self, node: usize) {
        if !self.adjacency_list.contains_key(&node) {
            self.adjacency_list.insert(node, Vec::new());
            self.nodes.push(node);
        }
    }
    
    pub fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.add_node(from);
        self.add_node(to);
        
        self.adjacency_list
            .get_mut(&from)
            .unwrap()
            .push((to, weight));
    }
    
    pub fn add_bidirectional_edge(&mut self, node1: usize, node2: usize, weight: usize) {
        self.add_edge(node1, node2, weight);
        self.add_edge(node2, node1, weight);
    }
    
    pub fn neighbors(&self, node: usize) -> &[(usize, usize)]  {
        // static EMPTY: Vec<(usize, usize)> = Vec::new();
        // self.adjacency_list.get(&node).unwrap_or(&EMPTY)
        self.adjacency_list.get(&node).map_or(&[], |v| v.as_slice())
    }
    
    pub fn nodes(&self) -> &Vec<usize> {
        &self.nodes
    }
    
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    
    pub fn edge_count(&self) -> usize {
        self.adjacency_list.values().map(|v| v.len()).sum()
    }
}
