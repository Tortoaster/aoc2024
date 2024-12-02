use itertools::Itertools;

pub fn solve_a(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|numbers| solve_a_line(numbers))
        .count() as u64
}

pub fn solve_b(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|numbers| {
            if solve_a_line(numbers) {
                return true;
            }
            for n in 0..numbers.len() {
                if solve_a_line(
                    &numbers
                        .iter()
                        .copied()
                        .enumerate()
                        .filter(|(index, _)| *index != n)
                        .map(|(_, value)| value)
                        .collect::<Vec<_>>(),
                ) {
                    return true;
                }
            }
            false
        })
        .count() as u64
}

fn solve_a_line(numbers: &[u64]) -> bool {
    let mut pairs = numbers
        .iter()
        .copied()
        .dropping_back(1)
        .zip(numbers.iter().copied().dropping(1))
        .peekable();

    let increasing = pairs.peek().map(|(a, b)| a < b).unwrap_or_default();

    pairs.all(|(a, b)| (increasing && a < b || !increasing && a > b) && safe_difference(a, b))
}

fn safe_difference(a: u64, b: u64) -> bool {
    (1..=3).contains(&a.abs_diff(b))
}

#[cfg(test)]
mod tests {
    use crate::day2::solve_a;

    #[test]
    fn test_a() {
        const INPUT: &str = "\
            7 6 4 2 1\n\
            1 2 7 8 9\n\
            9 7 6 2 1\n\
            1 3 2 4 5\n\
            8 6 4 4 1\n\
            1 3 6 7 9\n\
        ";

        assert_eq!(solve_a(INPUT), 2);
    }
}
