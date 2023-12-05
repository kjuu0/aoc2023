use std::io;

use day5::compute_lowest_location;

fn main() {
    println!("{}", compute_lowest_location(io::stdin().lock()));
}
