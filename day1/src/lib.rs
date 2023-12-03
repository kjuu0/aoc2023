use std::io::BufRead;

use aho_corasick::AhoCorasick;
use lazy_static::lazy_static;
use phf::phf_map;
use regex::Regex;

pub fn calibration_sum_pt1(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("failed to read line");
            let digits = line.chars().filter_map(|c| c.to_digit(/*radix=*/ 10));
            digits.clone().nth(0).expect("line has no digit") * 10 + digits.rev().nth(0).unwrap()
        })
        .sum()
}

lazy_static! {
    static ref DIGITS_RE: Regex =
        Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|[0-9])").unwrap();
    static ref REV_DIGITS_RE: Regex =
        Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|[0-9])").unwrap();
    static ref DIGITS_AHO: AhoCorasick = AhoCorasick::new(&[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9"
    ])
    .unwrap();
    static ref REV_DIGITS_AHO: AhoCorasick = AhoCorasick::new(&[
        "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin", "1", "2", "3", "4",
        "5", "6", "7", "8", "9"
    ])
    .unwrap();
}

// Potentially split reverse into separate map?
static DIGITS_MAP: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "eno" => 1,
    "1" => 1,
    "two" => 2,
    "owt" => 2,
    "2" => 2,
    "three" => 3,
    "eerht" => 3,
    "3" => 3,
    "four" => 4,
    "ruof" => 4,
    "4" => 4,
    "five" => 5,
    "evif" => 5,
    "5" => 5,
    "six" => 6,
    "xis" => 6,
    "6" => 6,
    "seven" => 7,
    "neves" => 7,
    "7" => 7,
    "eight" => 8,
    "thgie" => 8,
    "8" => 8,
    "nine" => 9,
    "enin" => 9,
    "9" => 9,
};

pub fn calibration_sum_pt2_regex(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .map(|line| {
            let line = line.expect("failed to read line");
            let rev_line = line.chars().rev().collect::<String>();
            DIGITS_RE
                .captures_iter(&line)
                .map(|digit| DIGITS_MAP.get(&digit[0]).unwrap())
                .nth(0)
                .expect("no digit in line")
                * 10
                + REV_DIGITS_RE
                    .captures_iter(&rev_line)
                    .map(|digit| DIGITS_MAP.get(&digit[0]).unwrap())
                    .nth(0)
                    .unwrap()
        })
        .sum()
}

pub fn calibration_sum_pt2_aho_corasick(reader: impl BufRead) -> u32 {
    reader.lines().map(|line| {
        let line = line.expect("failed to read line");
        // TODO: handrolled aho corasick to avoid reverse alloc?
        let rev_line = line.chars().rev().collect::<String>();
        (DIGITS_AHO
            .find_iter(&line)
            .nth(0)
            .expect("no digit in line")
            .pattern()
            .as_u32()
            % 9
            + 1)
            * 10
            + (REV_DIGITS_AHO
                .find_iter(&rev_line)
                .nth(0)
                .unwrap()
                .pattern()
                .as_u32()
                % 9
                + 1)
    }).sum()
}
