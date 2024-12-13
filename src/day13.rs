use std::ops::Add;

struct Coords(usize, usize);

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn solve_a(input: &str) -> u64 {
    let problems: Vec<_> = input
        .split("\n\n")
        .map(|triple| {
            let mut lines = triple.lines();
            let (left, right) = lines
                .next()
                .unwrap()
                .strip_prefix("Button A: ")
                .unwrap()
                .split_once(", ")
                .unwrap();
            let a = Coords(
                left.strip_prefix("X+").unwrap().parse().unwrap(),
                right.strip_prefix("Y+").unwrap().parse().unwrap(),
            );
            let (left, right) = lines
                .next()
                .unwrap()
                .strip_prefix("Button B: ")
                .unwrap()
                .split_once(", ")
                .unwrap();
            let b = Coords(
                left.strip_prefix("X+").unwrap().parse().unwrap(),
                right.strip_prefix("Y+").unwrap().parse().unwrap(),
            );
            let (left, right) = lines
                .next()
                .unwrap()
                .strip_prefix("Prize: ")
                .unwrap()
                .split_once(", ")
                .unwrap();
            let prize = Coords(
                left.strip_prefix("X=").unwrap().parse().unwrap(),
                right.strip_prefix("Y=").unwrap().parse().unwrap(),
            );
            (a, b, prize)
        })
        .collect();

    problems
        .into_iter()
        .filter_map(|(a, b, prize)| {
            let mut cost: Vec<Vec<_>> = (0..=prize.1)
                .map(|y| (0..=prize.0).map(|_| None).collect())
                .collect();
            cost[prize.1][prize.0] = Some(0u64);
            for y in (0..=prize.1).rev() {
                for x in (0..=prize.0).rev() {
                    if let Some(c) = cost
                        .get(y + a.1)
                        .and_then(|row| row.get(x + a.0).copied().flatten())
                    {
                        cost[y][x] = Some(c + 3);
                    }

                    if let Some(c) = cost
                        .get(y + b.1)
                        .and_then(|row| row.get(x + b.0).copied().flatten())
                    {
                        if cost[y][x].is_none() || cost[y][x].unwrap() > c + 1 {
                            cost[y][x] = Some(c + 1);
                        }
                    }
                }
            }

            cost[0][0]
        })
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::{solve_a, solve_b};

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

    #[test]
    fn test_b() {
        assert_eq!(solve_b(INPUT), 1206);
    }
}
