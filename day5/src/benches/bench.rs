use std::{fs::File, io::BufReader};

use criterion::{criterion_group, criterion_main, Criterion};
use day5::{compute_lowest_location, compute_lowest_location_seed_range};

// time:   [65.106 µs 65.383 µs 65.758 µs]
pub fn pt1_benchmark(c: &mut Criterion) {
    c.bench_function("compute_lowest_location official input", |b| {
        b.iter_batched_ref(
            || BufReader::new(File::open("src/benches/input.txt").unwrap()),
            |reader| assert_eq!(compute_lowest_location(reader), 174137457),
            criterion::BatchSize::SmallInput,
        );
    });
}

// bench too slow!!!!
// time:   90s
pub fn pt2_benchmark() {
    /*
    let reader = BufReader::new(File::open("src/benches/input.txt").unwrap());
    assert_eq!(compute_lowest_location_seed_range(reader), 1493866);
    */
}

criterion_group!(benches, pt1_benchmark);
criterion_main!(benches);
