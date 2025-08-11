// https://arxiv.org/pdf/2504.17033

fn findPivot(
    bound: usize, 
    graph: &crate::graph::Graph,
    s: &HashSet<usize>, // Set of source vertices
    k: usize, // Parameter k from the algorithm
    distances: &mut HashMap<usize, usize>
) -> (HashSet<usize>, HashSet<usize>) { // Returns (P, W)
    
    let mut w = s.clone(); // W ← S
    let mut w_prev = s.clone(); // W0 ← S
    
    // Relax for k steps
    for i in 1..=k {
        let mut w_i = HashSet::new(); // Wi ← ∅
        
        // For all edges (u,v) with u ∈ Wi-1
        for &u in &w_prev {
            for &(v, weight) in graph.neighbors(u) {
                let d_u = distances.get(&u).unwrap_or(&usize::MAX);
                let d_v = distances.get(&v).unwrap_or(&usize::MAX);
                
                // if d[u] + wuv ≤ d[v] then
                if d_u.saturating_add(weight) <= *d_v {
                    // d[v] ← d[u] + wuv
                    distances.insert(v, d_u.saturating_add(weight));
                    
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


pub fn bmssp(graph: &crate::graph::Graph, start: usize) -> HashMap<usize, usize> {
 
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
