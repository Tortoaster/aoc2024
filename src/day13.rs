use std::ops::Add;

struct Coords(i64, i64);

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub fn solve_a(input: &str) -> u64 {
    let problems = parse_input(input);
    problems
        .into_iter()
        .flat_map(|(a, b, prize)| solve(a, b, prize))
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    let problems = parse_input(input);
    problems
        .into_iter()
        .flat_map(|(a, b, prize)| {
            solve(
                a,
                b,
                Coords(prize.0 + 10000000000000, prize.1 + 10000000000000),
            )
        })
        .sum()
}

fn solve(a: Coords, b: Coords, prize: Coords) -> Result<u64, ()> {
    let q = prize.0 * a.1 - prize.1 * a.0;
    let r = b.0 * a.1 - b.1 * a.0;
    assert(|| q % r == 0)?;
    let b_presses = q / r;
    assert(|| b_presses >= 0)?;
    let s = prize.0 - b_presses * b.0;
    assert(|| s % a.0 == 0)?;
    let a_presses = s / a.0;
    assert(|| a_presses >= 0)?;
    Ok(a_presses as u64 * 3 + b_presses as u64)
}

fn assert(predicate: impl FnOnce() -> bool) -> Result<(), ()> {
    if predicate() {
        Ok(())
    } else {
        Err(())
    }
}

fn parse_input(input: &str) -> Vec<(Coords, Coords, Coords)> {
    input
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
