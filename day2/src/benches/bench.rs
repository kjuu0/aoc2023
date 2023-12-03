use std::{
    fs::File,
    io::BufReader,
};

use criterion::{criterion_group, criterion_main, Criterion};
use day2::{valid_game_sum, game_power_sum};

// time:   [29.525 µs 29.863 µs 30.290 µs]
pub fn pt1_benchmark(c: &mut Criterion) {
    c.bench_function("valid_game_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(valid_game_sum(reader), 2416),
            criterion::BatchSize::SmallInput,
        );
    });
}

// time:   [43.841 µs 44.617 µs 45.713 µs]
pub fn pt2_benchmark(c: &mut Criterion) {
    c.bench_function("game_power_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(game_power_sum(reader), 63307),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, pt1_benchmark, pt2_benchmark);
criterion_main!(benches);
