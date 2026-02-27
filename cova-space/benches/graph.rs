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

fn bench_graph_dense(c: &mut Criterion) {
    let mut group = c.benchmark_group("Graph Dense");

    // Complete graph on 100 vertices: C(100,2) = 4950 edges
    let vertices: HashSet<usize> = (0..100).collect();
    let mut edges: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..100 {
        for j in (i + 1)..100 {
            edges.insert((i, j));
        }
    }

    group.bench_function("undirected_instantiation_dense", |b| b.iter(|| {
        Graph::<usize, Undirected>::new(black_box(vertices.clone()), black_box(edges.clone()))
    }));

    group.finish();
}

fn bench_graph_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Graph Large");

    // 1000-vertex path graph
    let vertices: HashSet<usize> = (0..1000).collect();
    let edges: HashSet<(usize, usize)> = (0..999).map(|i| (i, i + 1)).collect();

    group.bench_function("undirected_instantiation_1000", |b| b.iter(|| {
        Graph::<usize, Undirected>::new(black_box(vertices.clone()), black_box(edges.clone()))
    }));

    let directed_graph = Graph::<usize, Directed>::new(vertices.clone(), edges.clone());

    group.bench_function("directed_contains_vertex_1000", |b| b.iter(|| {
        black_box(directed_graph.contains(black_box(&VertexOrEdge::Vertex(500))))
    }));

    group.finish();
}

criterion_group!(benches, bench_graph, bench_graph_dense, bench_graph_large);
criterion_main!(benches);
