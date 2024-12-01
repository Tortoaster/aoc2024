const INPUT: &str = include_str!("../inputs/input1a.txt");

fn main() {
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
    let sum = first
        .into_iter()
        .zip(second)
        .map(|(first, second)| first.abs_diff(second))
        .sum::<u32>();
    println!("{sum}");
}
