use std::{
    fs::File,
    io::BufReader,
};

use criterion::{criterion_group, criterion_main, Criterion};
use day4::card_points_sum;

// time:   [354.53 µs 360.42 µs 366.93 µs]
pub fn pt1_benchmark(c: &mut Criterion) {
    c.bench_function("card_points_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(card_points_sum(reader), 20107),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, pt1_benchmark);
criterion_main!(benches);
