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
                .any(|ops| perform::<2>(ops, numbers) == *result)
        })
        .map(|(result, _)| result)
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
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
            (0..3usize.pow(numbers.len() as u32 - 1))
                .into_par_iter()
                .any(|ops| perform::<3>(ops, numbers) == *result)
        })
        .map(|(result, _)| result)
        .sum()
}

fn perform<const NUM_OPS: usize>(mut ops: usize, numbers: &[u64]) -> u64 {
    let mut total = numbers[0];
    for number in numbers.iter().skip(1) {
        let op = ops % NUM_OPS;
        match op {
            0 => total += number,
            1 => total *= number,
            _ => total = 10u64.pow(number.ilog(10) + 1) * total + number,
        }
        ops /= NUM_OPS;
    }
    total
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
