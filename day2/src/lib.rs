use std::io::BufRead;

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
                // TODO: can maybe avoid a full parse by just checking first digit == 1 and just compare second digit char
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
