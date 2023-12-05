use std::{io, time::Instant};

use day5::{compute_lowest_location, compute_lowest_location_seed_range};

fn main() {
    let before = Instant::now();
    let res = compute_lowest_location_seed_range(io::stdin().lock());
    let end = before.elapsed();
    println!("res={}, time={}", res, end.as_secs());
}
