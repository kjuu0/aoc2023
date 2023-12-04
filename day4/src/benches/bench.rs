use std::{
    fs::File,
    io::BufReader,
};

use criterion::{criterion_group, criterion_main, Criterion};
use day4::{card_points_hashset_sum, card_points_linear_search_sum, scratchcards_interval_sum, scratchcards_sim_sum};

// time:   [354.53 µs 360.42 µs 366.93 µs]
pub fn pt1_hashset_benchmark(c: &mut Criterion) {
    c.bench_function("card_points_hashset_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(card_points_hashset_sum(reader), 20107),
            criterion::BatchSize::SmallInput,
        );
    });
}

// time:   [157.55 µs 157.97 µs 158.37 µs]
pub fn pt1_linear_benchmark(c: &mut Criterion) {
    c.bench_function("card_points_linear_search_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(card_points_linear_search_sum(reader), 20107),
            criterion::BatchSize::SmallInput,
        );
    });
}

// time:   [157.02 µs 158.89 µs 161.16 µs]
pub fn pt2_sim_benchmark(c: &mut Criterion) {
    c.bench_function("scratchcards_sim_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(scratchcards_sim_sum(reader), 8172507),
            criterion::BatchSize::SmallInput,
        );
    });
}

// time:   [167.24 µs 167.82 µs 168.59 µs]
pub fn pt2_interval_benchmark(c: &mut Criterion) {
    c.bench_function("scratchcards_interval_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(scratchcards_interval_sum(reader), 8172507),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, pt1_hashset_benchmark, pt1_linear_benchmark, pt2_sim_benchmark, pt2_interval_benchmark);
criterion_main!(benches);
