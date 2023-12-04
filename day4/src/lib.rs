use std::{collections::HashSet, io::BufRead};

pub fn card_points(card_str: &str) -> u32 {
    let nums = &card_str[card_str.find(':').expect("invalid card format")..];
    let (winning_nums, owned_nums) = nums.split_at(nums.find('|').expect("invalid card format"));
    // TODO: or just replace this with faster hashmap...
    // TODO: Given small N, replacing this with just linear search is probably faster...
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

pub fn card_points_sum(reader: impl BufRead) -> u32 {
    reader
        .lines()
        .map(|line| card_points(&line.expect("failed to read line")))
        .sum()
}
