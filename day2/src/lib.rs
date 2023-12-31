use std::io::BufRead;

/* part 1 */

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

pub fn is_valid_game(game_str: &str) -> bool {
    let seq = &game_str[(game_str.find(':').expect("invalid game format") + 2)..];
    seq.split(&[';', ',']).all(|mut pull| {
        if pull.as_bytes()[0] == b' ' {
            pull = &pull[1..];
        }
        pull.as_bytes()[1] == b' ' // Single digit case
            || (pull.as_bytes()[2] == b' ' // Double digit case
                && pull[0..2].parse::<u32>().expect("invalid game format") 
                <= match &pull[3..] {
                    "red" => RED,
                    "green" => GREEN,
                    "blue" => BLUE,
                    _ => panic!("invalid game format"),
                })
    })
}

pub fn valid_game_sum(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .zip(1..)
        .filter_map(|(line, game_id)| {
            if is_valid_game(&(line.expect("failed to read line"))) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

/* part 2 */
pub fn game_power(game_str: &str) -> u32 {
    let seq = &game_str[(game_str.find(':').expect("invalid game format") + 2)..];
    seq.split(&[';', ','])
        .fold([0; 3], |mut acc, mut pull| {
            if pull.as_bytes()[0] == b' ' {
                pull = &pull[1..];
            }
            let (num, color) = pull.split_at(pull.find(' ').expect("invalid game format"));
            let hash = match &color[1..] {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!("invalid game format"),
            };
            acc[hash] = acc[hash].max(num.parse::<u32>().unwrap());
            acc
        })
        .into_iter()
        .product()
}

pub fn game_power_sum(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .map(|line| game_power(&line.expect("failed to read line")))
        .sum()
}
