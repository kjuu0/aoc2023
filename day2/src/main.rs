use std::io;

use day2::valid_game_sum;

fn main() {
    println!("{}", valid_game_sum(io::stdin().lock()));
}
