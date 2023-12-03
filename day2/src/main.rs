use std::io;

use day2::{valid_game_sum, game_power_sum};

fn main() {
    println!("{}", game_power_sum(io::stdin().lock()));
}
