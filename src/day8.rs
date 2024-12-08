use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

pub fn solve_a(input: &str) -> u64 {
    let (antennas, (width, height)) = parse_input(input);

    let antinodes: BTreeSet<_> = antennas
        .values()
        .flat_map(|group| {
            group.iter().flat_map(|a| {
                group
                    .iter()
                    .filter(|&b| *a != *b)
                    .filter_map(|b| {
                        Some(((b.0 * 2).checked_sub(a.0)?, (b.1 * 2).checked_sub(a.1)?))
                    })
                    .filter(|&(x, y)| x < width && y < height)
            })
        })
        .collect();

    antinodes.len() as u64
}

pub fn solve_b(input: &str) -> u64 {
    let (antennas, (width, height)) = parse_input(input);

    let antinodes: BTreeSet<_> = antennas
        .values()
        .flat_map(|group| {
            group.iter().flat_map(|a| {
                group.iter().filter(|&b| *a != *b).flat_map(|b| {
                    (0..)
                        .map_while(|index| {
                            Some((
                                (b.0 * (index + 1)).checked_sub(a.0 * index)?,
                                (b.1 * (index + 1)).checked_sub(a.1 * index)?,
                            ))
                        })
                        .take_while(|&(x, y)| x < width && y < height)
                })
            })
        })
        .collect();

    antinodes.len() as u64
}

fn parse_input(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, (usize, usize)) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c != '.').then_some((c, (x, y))))
        })
        .into_grouping_map()
        .collect();

    (antennas, (width, height))
}
