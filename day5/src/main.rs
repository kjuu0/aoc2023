use std::{io, time::Instant};

use day5::{compute_lowest_location, compute_lowest_location_seed_range_brute, compute_lowest_location_seed_range_rev};

fn main() {
    let before = Instant::now();
    let res = compute_lowest_location_seed_range_brute(io::stdin().lock());
    // let res = compute_lowest_location_seed_range_rev(io::stdin().lock());
    let end = before.elapsed();
    println!("res={}, time={}", res, end.as_micros());
}
