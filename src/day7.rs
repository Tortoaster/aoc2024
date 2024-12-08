use rayon::prelude::*;

pub fn solve_a(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(result, numbers)| {
            (
                result.parse::<u64>().unwrap(),
                numbers
                    .split(' ')
                    .map(|n| n.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(result, numbers)| {
            (0usize..1 << (numbers.len() - 1))
                .into_par_iter()
                .any(|ops| perform(ops, numbers) == *result)
        })
        .map(|(result, _)| result)
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    0
}

fn perform(ops: usize, numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .skip(1)
        .zip((0..numbers.len() - 1).map(|shift| (ops >> shift) & 1).rev())
        .fold(numbers[0], |acc, (number, op)| {
            if op == 0 {
                acc + number
            } else {
                acc * number
            }
        })
}

#[cfg(test)]
mod tests {
    use super::solve_a;

    #[test]
    fn test_a() {
        const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

        assert_eq!(solve_a(INPUT), 3749);
    }
}
