use std::collections::BTreeMap;

mod day1;
mod day2;
mod day3;

fn main() {
    let mut days: BTreeMap<u32, (&str, fn(&str) -> u64, fn(&str) -> u64)> = BTreeMap::new();

    days.insert(
        1,
        (
            include_str!("../input/day1.txt"),
            day1::solve_a,
            day1::solve_b,
        ),
    );
    days.insert(
        2,
        (
            include_str!("../input/day2.txt"),
            day2::solve_a,
            day2::solve_b,
        ),
    );
    days.insert(
        3,
        (
            include_str!("../input/day3.txt"),
            day3::solve_a,
            day3::solve_b,
        ),
    );

    for (day, (input, solve_a, solve_b)) in days {
        println!("Day {day}a: {}", solve_a(input));
        println!("Day {day}b: {}", solve_b(input));
    }
}
