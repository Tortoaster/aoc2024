use cached::proc_macro::cached;

pub fn solve_a(input: &str) -> u64 {
    blink(input, 25)
}

pub fn solve_b(input: &str) -> u64 {
    blink(input, 75)
}

fn blink(input: &str, times: usize) -> u64 {
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .map(|n| count_stones(n, times))
        .sum()
}

#[cached]
fn count_stones(number: u64, blinks_left: usize) -> u64 {
    if blinks_left == 0 {
        1
    } else if number == 0 {
        count_stones(1, blinks_left - 1)
    } else if number.ilog10() & 1 == 1 {
        let div = 10u64.pow(number.ilog10() / 2 + 1);
        let left = number / div;
        let right = number % div;
        count_stones(left, blinks_left - 1) + count_stones(right, blinks_left - 1)
    } else {
        count_stones(number * 2024, blinks_left - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::solve_a;

    const INPUT: &str = "125 17";

    #[test]
    fn test_a() {
        assert_eq!(solve_a(INPUT), 55312);
    }
}
