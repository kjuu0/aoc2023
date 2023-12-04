use std::io;

use day4::{card_points_hashset_sum, scratchcards_interval_sum};

fn main() {
    println!("{}", scratchcards_interval_sum(io::stdin().lock()));
}
