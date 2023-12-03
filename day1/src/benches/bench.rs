use std::{
    fs::File,
    io::BufReader,
};

use criterion::{criterion_group, criterion_main, Criterion};
use day1::{calibration_sum_pt1, calibration_sum_pt2};

// time:   [106.71 µs 107.02 µs 107.38 µs]
pub fn pt1_benchmark(c: &mut Criterion) {
    c.bench_function("calibration_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(calibration_sum_pt1(reader), 54916),
            criterion::BatchSize::SmallInput,
        );
    });
}

// time:   [909.78 µs 914.92 µs 920.64 µs]
pub fn pt2_benchmark(c: &mut Criterion) {
    c.bench_function("calibration_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(calibration_sum_pt2(reader), 54728),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, pt1_benchmark, pt2_benchmark);
criterion_main!(benches);
