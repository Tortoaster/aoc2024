use std::{collections::BTreeSet, io, io::Write};

use itertools::Itertools;

type Robot = (isize, isize, isize, isize);
type Pos = (usize, usize);

pub fn solve_a(input: &str) -> u64 {
    let robots = parse_input(input);

    let positions: Vec<_> = robots
        .into_iter()
        .map(|robot| pass_seconds(&robot, 100, 101, 103))
        .collect();

    safety_factor(&positions, 101, 103)
}

pub fn solve_b(input: &str) -> u64 {
    let robots = parse_input(input);

    for seconds in 100.. {
        let positions: BTreeSet<_> = robots
            .iter()
            .map(|robot| pass_seconds(robot, seconds, 101, 103))
            .collect();

        let significant = (0..101)
            .flat_map(|y| (0..99).map(move |x| (x, y)))
            .any(|(x, y)| {
                (0..3)
                    .flat_map(|dy| (0..3).map(move |dx| (dx, dy)))
                    .all(|(dx, dy)| positions.contains(&(x + dx, y + dy)))
            });

        if significant {
            for y in 0..103 {
                for x in 0..101 {
                    let number = positions
                        .iter()
                        .filter(|pos| pos.0 == x && pos.1 == y)
                        .count();
                    print!(
                        "{}",
                        if number > 0 {
                            number.to_string()
                        } else {
                            ".".to_owned()
                        }
                    );
                }
                println!();
            }

            print!("Does this look like a christmas tree? [y/N] ");
            io::stdout().flush().unwrap();
            let mut input: String = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" {
                return seconds as u64;
            }
        }
    }

    0
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .flat_map(|line| line.split_whitespace())
        .flat_map(|coords| coords[2..].split(','))
        .filter_map(|n| n.parse().ok())
        .tuples()
        .collect()
}

fn pass_seconds(robot: &Robot, seconds: isize, width: usize, height: usize) -> Pos {
    (
        (robot.0 + robot.2 * seconds).rem_euclid(width as isize) as usize,
        (robot.1 + robot.3 * seconds).rem_euclid(height as isize) as usize,
    )
}

fn safety_factor(positions: &[Pos], width: usize, height: usize) -> u64 {
    let mut top_left = Vec::new();
    let mut top_right = Vec::new();
    let mut bottom_left = Vec::new();
    let mut bottom_right = Vec::new();

    for pos in positions {
        if pos.0 < width / 2 {
            if pos.1 < height / 2 {
                top_left.push(*pos);
            } else if pos.1 > height / 2 {
                bottom_left.push(*pos);
            }
        } else if pos.0 > width / 2 {
            if pos.1 < height / 2 {
                top_right.push(*pos);
            } else if pos.1 > height / 2 {
                bottom_right.push(*pos);
            }
        }
    }

    (top_left.len() * top_right.len() * bottom_left.len() * bottom_right.len()) as u64
}