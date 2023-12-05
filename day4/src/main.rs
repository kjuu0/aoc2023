use std::{io::{self, BufReader}, fs::File};

use day4::{card_points_hashset_sum, scratchcards_interval_sum, scratchcards_sim_sum};

fn sim() {
    let reader =  BufReader::new(File::open("src/benches/input.txt").unwrap());
    println!("{}", scratchcards_sim_sum(reader))
}

fn interval() {
    let reader = BufReader::new(File::open("src/benches/input.txt").unwrap());
    println!("{}", scratchcards_interval_sum(reader))
}

fn main() {
    sim();
    interval();
}
