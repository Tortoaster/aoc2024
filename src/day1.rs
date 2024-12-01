use itertools::Itertools;

pub fn solve_1a(input: &str) -> u32 {
    let (mut first, mut second) = separate_lists(input);

    first.sort();
    second.sort();

    first
        .into_iter()
        .zip(second)
        .map(|(first, second)| first.abs_diff(second))
        .sum()
}

pub fn solve_1b(input: &str) -> u32 {
    let (first, second) = separate_lists(input);

    let map = second
        .into_iter()
        .map(|key| (key, 1))
        .into_grouping_map()
        .sum();

    first
        .into_iter()
        .map(|n| n * map.get(&n).copied().unwrap_or(0))
        .sum()
}

fn separate_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap()))
        .unzip()
}
