use std::io::BufRead;

pub fn calibration_sum(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("failed to read line");
            let digits = line.chars().filter_map(|c| c.to_digit(/*radix=*/ 10));
            digits.clone().nth(0).expect("line has no digit") * 10 + digits.rev().nth(0).unwrap()
        })
        .sum()
}
