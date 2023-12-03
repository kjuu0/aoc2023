use std::{
    fs::File,
    io::BufReader,
};

use criterion::{criterion_group, criterion_main, Criterion};
use day1::calibration_sum;

// time:   [2.3729 µs 2.3864 µs 2.4028 µs]
pub fn benchmark(c: &mut Criterion) {
    let file = File::open("src/benches/input.txt").unwrap();
    c.bench_function("calibration_sum official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(file.try_clone().unwrap()),
            |reader| calibration_sum(reader),
            criterion::BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
