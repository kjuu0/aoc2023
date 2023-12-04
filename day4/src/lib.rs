use core::num;
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    io::BufRead,
};

pub fn card_points_hashset(card_str: &str) -> u32 {
    let nums = &card_str[card_str.find(':').expect("invalid card format")..];
    let (winning_nums, owned_nums) = nums.split_at(nums.find('|').expect("invalid card format"));
    // TODO: or just replace this with faster hashmap...
    let num_match = winning_nums
        .split_whitespace()
        .flat_map(|num| num.parse::<u32>())
        .collect::<HashSet<u32>>()
        .intersection(
            &owned_nums
                .split_whitespace()
                .flat_map(|num| num.parse::<u32>())
                .collect::<HashSet<u32>>(),
        )
        .count() as u32;
    if num_match == 0 {
        0
    } else {
        2_u32.pow(num_match - 1)
    }
}

pub fn num_matches_linear_search(card_str: &str) -> u32 {
    let nums = &card_str[card_str.find(':').expect("invalid card format")..];
    let (winning_nums, owned_nums) = nums.split_at(nums.find('|').expect("invalid card format"));
    let winning_nums = winning_nums
        .split_whitespace()
        .flat_map(|num| num.parse::<u32>())
        .collect::<Vec<u32>>();
    owned_nums
        .split_whitespace()
        .flat_map(|num| num.parse::<u32>())
        .filter(|num| winning_nums.contains(num))
        .count() as u32
}

pub fn card_points_linear_search(card_str: &str) -> u32 {
    let num_match = num_matches_linear_search(card_str);
    if num_match == 0 {
        0
    } else {
        2_u32.pow(num_match - 1)
    }
}

pub fn card_points_hashset_sum(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .map(|line| card_points_hashset(&line.expect("failed to read line")))
        .sum()
}

pub fn card_points_linear_search_sum(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .map(|line| card_points_linear_search(&line.expect("failed to read line")))
        .sum()
}

// TODO: compare with "easy" array solution
pub fn scratchcards_sum(reader: impl BufRead) -> u32 {
    let mut multipliers = BTreeMap::new();
    let mut mul = 1;
    let mut sum = 0;
    for (i, line) in reader.lines().enumerate() {
        sum += mul;
        let matches = num_matches_linear_search(&line.expect("failed to read line"));
        if matches != 0 {
            *multipliers.entry(i + matches as usize).or_default() += mul;
            mul *= 2;
        }
        if let Some(dmul) = multipliers.get(&i) {
            mul -= dmul;
        }
    }
    sum
}
