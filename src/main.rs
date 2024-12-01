use crate::day1::{solve_1a, solve_1b};

mod day1;

const DAY1: &str = include_str!("../inputs/day1.txt");

fn main() {
    println!("Day 1a: {}", solve_1a(DAY1));
    println!("Day 1b: {}", solve_1b(DAY1));
}
