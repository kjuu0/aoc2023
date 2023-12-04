use std::{collections::HashSet, io::BufRead};

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

pub fn card_points_linear_search(card_str: &str) -> u32 {
    let nums = &card_str[card_str.find(':').expect("invalid card format")..];
    let (winning_nums, owned_nums) = nums.split_at(nums.find('|').expect("invalid card format"));
    let winning_nums = winning_nums
        .split_whitespace()
        .flat_map(|num| num.parse::<u32>())
        .collect::<Vec<u32>>();
    let num_match = owned_nums
        .split_whitespace()
        .flat_map(|num| num.parse::<u32>())
        .filter(|num| winning_nums.contains(num))
        .count() as u32;
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
