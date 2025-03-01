pub fn solve_a(input: &str) -> u64 {
    input
        .lines()
        .map(parse_numbers)
        .filter(|numbers| solve_a_line(numbers))
        .count() as u64
}

pub fn solve_b(input: &str) -> u64 {
    input
        .lines()
        .map(parse_numbers)
        .filter(|numbers| {
            solve_a_line(numbers)
                || (0..numbers.len())
                    .map(|n| numbers_without(n, numbers))
                    .any(|numbers| solve_a_line(&numbers))
        })
        .count() as u64
}

fn solve_a_line(numbers: &[u64]) -> bool {
    (numbers.is_sorted() || numbers.is_sorted_by(|a, b| b < a))
        && numbers
            .windows(2)
            .all(|window| (1u64..=3).contains(&window[0].abs_diff(window[1])))
}

fn parse_numbers(s: &str) -> Vec<u64> {
    s.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn numbers_without(n: usize, numbers: &[u64]) -> Vec<u64> {
    numbers
        .iter()
        .copied()
        .enumerate()
        .filter(|(index, _)| *index != n)
        .map(|(_, value)| value)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day2::solve_a;

    #[test]
    fn test_a() {
        const INPUT: &str = "\
            7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n";

        assert_eq!(solve_a(INPUT), 2);
    }
}
