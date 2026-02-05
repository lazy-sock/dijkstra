use criterion::{Criterion, black_box, criterion_group, criterion_main};
use dijkstra::{
    dijkstra,
    graph::{Graph, get_random_graph},
};

fn dijkstra_bench_random(c: &mut Criterion) {
    let graph = get_random_graph();
    c.bench_function("dijkstra", |b| b.iter(|| dijkstra(black_box(&graph), 0)));
}

criterion_group!(benches, dijkstra_bench_random);
criterion_main!(benches);
