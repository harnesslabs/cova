use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cova_space::lattice::Lattice;
use cova_space::set::Poset;

fn bench_lattice(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lattice");

    group.bench_function("add_element", |b| b.iter(|| {
        let mut lattice = Lattice::new();
        lattice.add_element(black_box(1));
        lattice.add_element(black_box(2));
        black_box(lattice)
    }));

    group.bench_function("add_relation", |b| b.iter(|| {
        let mut lattice = Lattice::new();
        lattice.add_relation(black_box(1), black_box(2));
        lattice.add_relation(black_box(2), black_box(3));
        black_box(lattice)
    }));

    // Setup for transitive closure and poset checks
    let mut initial_lattice = Lattice::new();
    for i in 0..50 {
        initial_lattice.add_relation(i, i + 1);
    }

    /*
    group.bench_function("compute_transitive_closure", |b| b.iter(|| {
        let mut lattice = initial_lattice.clone();
        lattice.compute_transitive_closure();
        black_box(lattice)
    }));
    */

    group.bench_function("leq", |b| b.iter(|| {
        // We aren't testing on a fully transitively closed lattice because 
        // compute_transitive_closure is private, but we can test the `leq` execution path.
        black_box(initial_lattice.leq(black_box(&0), black_box(&25)))
    }));

    group.finish();
}

fn bench_lattice_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lattice Large");

    // Building a 500-element chain
    group.bench_function("add_relation_500", |b| b.iter(|| {
        let mut lattice = Lattice::new();
        for i in 0..500 {
            lattice.add_relation(black_box(i), black_box(i + 1));
        }
        black_box(lattice)
    }));

    // leq traversal over a 500-element chain
    let mut large_lattice = Lattice::new();
    for i in 0..500 {
        large_lattice.add_relation(i, i + 1);
    }

    group.bench_function("leq_500", |b| b.iter(|| {
        black_box(large_lattice.leq(black_box(&0), black_box(&250)))
    }));

    group.finish();
}

criterion_group!(benches, bench_lattice, bench_lattice_large);
criterion_main!(benches);
