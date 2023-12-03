use std::{
    fs::File,
    io::BufReader,
};

use criterion::{criterion_group, criterion_main, Criterion};
use day2::valid_game_sum;

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

criterion_group!(benches, pt1_benchmark);
criterion_main!(benches);
