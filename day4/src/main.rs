use std::io;

use day4::card_points_sum;

fn main() {
    println!("{}", card_points_sum(io::stdin().lock()));
}
