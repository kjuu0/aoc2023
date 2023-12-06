use std::{fs::File, io::BufReader};

use criterion::{criterion_group, criterion_main, Criterion};
use day6::{winning_ways_product, winning_ways};

// time:   [6.8863 µs 6.9177 µs 6.9469 µs]
pub fn pt1_benchmark(c: &mut Criterion) {
    c.bench_function("winning_ways_product official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(winning_ways_product(reader), 281600),
            criterion::BatchSize::SmallInput,
        );
    });
}

// time:   [6.9682 µs 7.0455 µs 7.1174 µs]
pub fn pt2_benchmark(c: &mut Criterion) {
    c.bench_function("winning_ways official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(winning_ways(reader), 33875953),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, pt1_benchmark, pt2_benchmark);
criterion_main!(benches);
