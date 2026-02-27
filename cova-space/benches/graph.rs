use std::collections::HashSet;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_space::graph::{Directed, Graph, Undirected, VertexOrEdge};
use cova_space::set::Collection;

fn bench_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("Graph");

    let vertices: HashSet<usize> = (0..100).collect();
    let edges: HashSet<(usize, usize)> = (0..99).map(|i| (i, i + 1)).collect();

    group.bench_function("undirected_instantiation", |b| b.iter(|| {
        Graph::<usize, Undirected>::new(black_box(vertices.clone()), black_box(edges.clone()))
    }));

    let undirected_graph = Graph::<usize, Undirected>::new(vertices.clone(), edges.clone());

    group.bench_function("undirected_contains_vertex", |b| b.iter(|| {
        black_box(undirected_graph.contains(black_box(&VertexOrEdge::Vertex(50))))
    }));

    group.bench_function("undirected_contains_edge", |b| b.iter(|| {
        black_box(undirected_graph.contains(black_box(&VertexOrEdge::Edge(50, 51))))
    }));

    group.bench_function("directed_instantiation", |b| b.iter(|| {
        Graph::<usize, Directed>::new(black_box(vertices.clone()), black_box(edges.clone()))
    }));

    let directed_graph = Graph::<usize, Directed>::new(vertices.clone(), edges.clone());

    group.bench_function("directed_contains_vertex", |b| b.iter(|| {
        black_box(directed_graph.contains(black_box(&VertexOrEdge::Vertex(50))))
    }));

    group.bench_function("directed_contains_edge", |b| b.iter(|| {
        black_box(directed_graph.contains(black_box(&VertexOrEdge::Edge(50, 51))))
    }));

    group.finish();
}

criterion_group!(benches, bench_graph);
criterion_main!(benches);
