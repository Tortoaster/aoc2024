use itertools::Itertools;
use rayon::prelude::*;
use std::ops::{Add, Div, Mul, Rem};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Pos([usize; 2]);

impl Pos {
    fn as_slice(&self) -> &[usize; 2] {
        &self.0
    }
}

impl Mul<usize> for Pos {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self([self.0[0] * rhs, self.0[1] * rhs])
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1]])
    }
}

impl Div for Pos {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self([self.0[0] / rhs.0[0], self.0[1] / rhs.0[1]])
    }
}

impl Rem for Pos {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self([self.0[0] % rhs.0[0], self.0[1] % rhs.0[1]])
    }
}

pub fn solve_a(input: &str) -> u64 {
    // let machines: Vec<(Pos, Pos, Pos)> = parse_input(input);
    //
    // machines
    //     .into_iter()
    //     .filter_map(|(a, b, prize)| {
    //         let mut cost: Vec<Vec<_>> = (0..=prize.1)
    //             .map(|y| (0..=prize.0).map(|_| None).collect())
    //             .collect();
    //         cost[prize.1][prize.0] = Some(0u64);
    //
    //         for y in (0..=prize.1).rev() {
    //             for x in (0..=prize.0).rev() {
    //                 cost[y][x] = [(a, 3), (b, 1)]
    //                     .iter()
    //                     .filter_map(|&(Pos(dx, dy), tokens)| {
    //                         cost.get(y + dy)
    //                             .and_then(|row| row.get(x + dx).copied().flatten())
    //                             .map(|cost| cost + tokens)
    //                     })
    //                     .min();
    //             }
    //         }
    //
    //         cost[0][0]
    //     })
    //     .sum()

    0
}

pub fn solve_b(input: &str) -> u64 {
    let machines: Vec<(Pos, Pos, Pos)> = parse_input(input);
    machines
        .into_par_iter()
        .filter_map(|(a, b, prize)| {
            let divider = (a.0[0] * b.0[1]) as isize - (b.0[0] * a.0[1]) as isize;
            match dbg!(divider) {
                0 => None,
                _ => {
                    let b_counter = (a.0[0] * prize.0[1]) as isize - (prize.0[0] * a.0[1]) as isize;
                    if b_counter % divider == 0 {
                        let b_presses = dbg!(b_counter / divider);
                        if b_presses >= 0 {
                            let b_presses = b_presses as usize;
                            let a_presses_count = prize.0[0] - b_presses * b.0[0];
                            if a_presses_count % a.0[0] == 0 {
                                let a_presses = a_presses_count / a.0[0];
                                if a_presses * a.0[1] + b_presses * b.0[1] == prize.0[1] {
                                    Some(a_presses * 3 + b_presses)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }
        })
        .sum::<usize>() as u64
}

fn parse_input(input: &str) -> Vec<(Pos, Pos, Pos)> {
    input
        .lines()
        .filter_map(|line| {
            let (_, line) = line.split_once(": ")?;
            let (left, right) = line.split_once(", ")?;
            Some(Pos([left[2..].parse().ok()?, right[2..].parse().ok()?]))
        })
        .tuples()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::solve_a;

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn test_a() {
        assert_eq!(solve_a(INPUT), 480);
    }
}
