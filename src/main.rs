use std::collections::BTreeMap;

fn main() {
    solve_1b();
}

fn solve_1a() {
    const INPUT: &str = include_str!("../inputs/input1.txt");
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = INPUT
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(first, second)| {
            (
                first.parse::<i32>().unwrap(),
                second.parse::<i32>().unwrap(),
            )
        })
        .unzip();
    first.sort();
    second.sort();
    let sum: u32 = first
        .into_iter()
        .zip(second)
        .map(|(first, second)| first.abs_diff(second))
        .sum();
    println!("{sum}");
}

fn solve_1b() {
    const INPUT: &str = include_str!("../inputs/input1.txt");
    let (first, mut second): (Vec<_>, Vec<_>) = INPUT
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(first, second)| {
            (
                first.parse::<u32>().unwrap(),
                second.parse::<u32>().unwrap(),
            )
        })
        .unzip();
    second.sort();
    let mut map = BTreeMap::new();
    for n in second {
        map.entry(n).and_modify(|count| *count += 1).or_insert(1);
    }
    let sum: u32 = first
        .into_iter()
        .map(|n| n * *map.get(&n).unwrap_or(&0))
        .sum();
    println!("{sum}");
}
