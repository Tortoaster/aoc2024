use std::collections::BTreeMap;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

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
    days.insert(
        4,
        (
            include_str!("../input/day4.txt"),
            day4::solve_a,
            day4::solve_b,
        ),
    );
    days.insert(
        5,
        (
            include_str!("../input/day5.txt"),
            day5::solve_a,
            day5::solve_b,
        ),
    );
    // Takes too long
    // days.insert(
    //     6,
    //     (
    //         include_str!("../input/day6.txt"),
    //         day6::solve_a,
    //         day6::solve_b,
    //     ),
    // );
    days.insert(
        7,
        (
            include_str!("../input/day7.txt"),
            day7::solve_a,
            day7::solve_b,
        ),
    );

    for (day, (input, solve_a, solve_b)) in days {
        println!("Day {day}a: {}", solve_a(input));
        println!("Day {day}b: {}", solve_b(input));
    }
}
