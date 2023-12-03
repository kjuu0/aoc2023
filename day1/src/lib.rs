use std::io::BufRead;

pub fn calibration_sum(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("failed to read line");
            line.chars()
                .filter_map(|c| c.to_digit(/*radix=*/ 10))
                .nth(0)
                .expect("line has no digit")
                * 10
                + line
                    .chars()
                    .rev()
                    .filter_map(|c| c.to_digit(/*radix=*/ 10))
                    .nth(0)
                    .unwrap()
        })
        .sum()
}
