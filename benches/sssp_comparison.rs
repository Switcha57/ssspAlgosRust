use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use sssp_algos::{dijkstra, bfs, generate_random_graph};

fn benchmark_algorithms(c: &mut Criterion) {
    let mut group = c.benchmark_group("SSSP Algorithms");
    
    // Test different graph sizes
    let sizes = [10, 50, 100, 500];
    let edge_density = 0.3;
    
    for size in sizes.iter() {
        let graph = generate_random_graph(*size, edge_density);
        let start_node = 0;
        
        group.bench_with_input(
            BenchmarkId::new("Dijkstra", size),
            size,
            |b, _| {
                b.iter(|| {
                    dijkstra(black_box(&graph), black_box(start_node))
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("New Algorithm", size),
            size,
            |b, _| {
                b.iter(|| {
                    bfs(black_box(&graph), black_box(start_node))
                })
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_algorithms);
criterion_main!(benches);
