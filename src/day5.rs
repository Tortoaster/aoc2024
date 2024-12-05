use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

pub fn solve_a(input: &str) -> u64 {
    let (rules, pages) = parse_input(input);

    pages
        .into_iter()
        .filter(|page| conform_rules(page, &rules))
        .map(|page| page[page.len() / 2])
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    let (rules, pages) = parse_input(input);

    pages
        .into_iter()
        .filter(|page| !conform_rules(page, &rules))
        .map(|mut page| {
            page.sort_by(|a, b| compare_with_rules(a, b, &rules));
            page
        })
        .map(|page| page[page.len() / 2])
        .sum()
}

fn parse_input(input: &str) -> (HashMap<u64, HashSet<u64>>, Vec<Vec<u64>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules: HashMap<u64, HashSet<u64>> = rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
        .into_grouping_map()
        .collect();
    let pages: Vec<_> = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (rules, pages)
}

fn conform_rules(page: &[u64], rules: &HashMap<u64, HashSet<u64>>) -> bool {
    for (before, rest) in rules {
        for after in rest {
            match page.iter().position(|n| *n == *before) {
                None => {}
                Some(before) => match page.iter().position(|n| *n == *after) {
                    None => {}
                    Some(after) => {
                        if before > after {
                            return false;
                        }
                    }
                },
            }
        }
    }

    true
}

fn compare_with_rules(a: &u64, b: &u64, rules: &HashMap<u64, HashSet<u64>>) -> Ordering {
    match rules.get(a) {
        None => Ordering::Equal,
        Some(after) => {
            if after.contains(b) {
                Ordering::Less
            } else {
                match rules.get(b) {
                    None => Ordering::Equal,
                    Some(after) => {
                        if after.contains(a) {
                            Ordering::Greater
                        } else {
                            Ordering::Equal
                        }
                    }
                }
            }
        }
    }
}
