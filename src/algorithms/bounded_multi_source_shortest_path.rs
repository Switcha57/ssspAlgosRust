// https://arxiv.org/pdf/2504.17033

use std::collections::{ BinaryHeap, HashMap, HashSet};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::cmp::Ordering;
lazy_static! {
    static ref DISTANCES: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

// Calculate k := ⌊log₁/₃(n)⌋ 
fn calculate_k(n: usize) -> usize {
    if n <= 1 { return 1; }
    ((n as f64).ln() / (1.0/3.0_f64).ln()).floor() as usize
}

// Calculate t := ⌊log₂/₃(n)⌋
fn calculate_t(n: usize) -> usize {
    if n <= 1 { return 1; }
    ((n as f64).ln() / (2.0/3.0_f64).ln()).floor() as usize
}

fn findPivot(
    bound: usize, 
    graph: &crate::graph::Graph,
    s: &HashSet<usize>, // Set of source vertices
) -> (HashSet<usize>, HashSet<usize>) { // Returns (P, W)
    let n = graph.nodes().len();
    let k = calculate_k(n); // k := ⌊log₁/₃(n)⌋
    let mut distances = DISTANCES.lock().unwrap();
    
    let mut w = s.clone(); // W ← S
    let mut w_prev = s.clone(); // W0 ← S
    
    // Relax for k steps
    for i in 1..=k {
        let mut w_i = HashSet::new(); // Wi ← ∅
        
        // For all edges (u,v) with u ∈ Wi-1
        for &u in &w_prev {
            for &(v, weight) in graph.neighbors(u) {
                let d_u = *distances.get(&u).unwrap_or(&usize::MAX);
                let d_v = distances.get(&v).unwrap_or(&usize::MAX);
                
                // if d[u] + wuv ≤ d[v] then
                if d_u.saturating_add(weight) <= *d_v {
                    // d[v] ← d[u] + wuv
                    distances.insert(v, d_u + weight);
                    
                    // if d[u] + wuv < B then
                    if d_u.saturating_add(weight) < bound {
                        // Wi ← Wi ∪ {v}
                        w_i.insert(v);
                    }
                }
            }
        }
        
        // W ← W ∪ Wi
        w.extend(&w_i);
        w_prev = w_i;
        
        // if |W| > k|S| then
        if w.len() > k * s.len() {
            // P ← S
            return (s.clone(), w);
        }
    }
    
    // F ← {(u,v) ∈ E : u,v ∈ W, d[v] = d[u] + wuv}
    // F is a directed forest under Assumption 2.1
    let mut forest_edges: Vec<(usize, usize)> = Vec::new();
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
    
    for &u in &w {
        for &(v, weight) in graph.neighbors(u) {
            if w.contains(&v) {
                let d_u = distances.get(&u).unwrap_or(&usize::MAX);
                let d_v = distances.get(&v).unwrap_or(&usize::MAX);
                
                if *d_v == d_u.saturating_add(weight) {
                    forest_edges.push((u, v));
                    children.entry(u).or_insert_with(Vec::new).push(v);
                }
            }
        }
    }
    
    // P ← {u ∈ S : u is a root of a tree with ≥ k vertices in F}
    let mut p = HashSet::new();
    
    for &u in s {
        if count_tree_vertices(&children, u) >= k {
            p.insert(u);
        }
    }
    
    (p, w)
}

fn count_tree_vertices(children: &HashMap<usize, Vec<usize>>, root: usize) -> usize{

        let mut count = 1;

        if let Some(child) = children.get(&root)  {
            for &node in child{
                count += count_tree_vertices(children, node);
            }
        }
        count
}
fn baseCase(boundary: usize, singleton: usize, graph: &crate::graph::Graph) -> (usize, HashSet<usize>) {
    let mut distances = DISTANCES.lock().unwrap();
    let mut Uo = HashSet::new();
    Uo.insert(singleton);
    let n = graph.nodes().len();
    let k = calculate_k(n); // k := ⌊log₁/₃(n)⌋
    let mut H = BinaryHeap::new();
    let mut in_heap = HashSet::new(); // Track which nodes are in heap
    
    H.push(State{cost: distances[&singleton], node: singleton });
    in_heap.insert(singleton);
    
    while !H.is_empty() && Uo.len() < k + 1 {
        let State { cost: d_u, node: u } = H.pop().unwrap();
        
        // Skip if this is an outdated entry
        if d_u > distances[&u] {
            continue;
        }
        
        in_heap.remove(&u);
        Uo.insert(u);
        
        for &(v, weight) in graph.neighbors(u) {
            let d_v = distances.get(&v).unwrap_or(&usize::MAX);
            let new_distance = d_u.saturating_add(weight);
            
            if new_distance <= *d_v && new_distance < boundary {
                distances.insert(v, new_distance);
                
                // if v is not in H then H.Insert(⟨v, d[v]⟩)
                if !in_heap.contains(&v) {
                    H.push(State { cost: new_distance, node: v });
                    in_heap.insert(v);
                } else {
                    // else H.DecreaseKey(⟨v, d[v]⟩)
                    // Since BinaryHeap doesn't support decrease-key directly,
                    // we insert the new value and ignore outdated entries when popping
                    H.push(State { cost: new_distance, node: v });
                }
            }
        }
    }

//      if |U0| ≤ k then
//          return B′ ← B,U ← U0
//          else
//          return B′ ← maxv∈U0 d[v],U ← {v ∈ U0 : d[v] < B′}
    if Uo.len() <= k {

        (boundary, Uo)
    } else {
        let max_distance = Uo.iter().map(|&v| distances[&v]).max().unwrap_or(usize::MAX);
        let filtered: HashSet<_> = Uo.into_iter().filter(|&v| distances[&v] < max_distance).collect();
        (max_distance, filtered)
    }


}
pub fn bmssp(graph: &crate::graph::Graph, start: usize) -> HashMap<usize, usize> {
    let n = graph.nodes().len();
    let k = calculate_k(n); // k := ⌊log₁/₃(n)⌋  
    let t = calculate_t(n); // t := ⌊log₂/₃(n)⌋
    
    let mut distances = DISTANCES.lock().unwrap();
    distances.clear(); // Reset global distances for this run
    
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: usize,
}

impl Ord for State { // min-heap
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}