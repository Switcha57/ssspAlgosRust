pub mod algorithms;
pub mod graph;
pub mod utils;

pub use algorithms::{dijkstra, new_algorithm};
pub use graph::Graph;
pub use utils::{generate_random_graph, BenchmarkResult};
