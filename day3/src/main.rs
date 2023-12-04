use std::io;

use day3::{parts_sum, gear_ratio_sum};

fn main() {
    println!("{}", gear_ratio_sum(io::stdin().lock()));
}
