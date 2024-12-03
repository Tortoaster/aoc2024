pub fn solve_a(input: &str) -> u64 {
    input.split("mul(").skip(1).filter_map(parse_args).sum()
}

pub fn solve_b(input: &str) -> u64 {
    input
        .split("do()")
        .map(|line| {
            line.split_once("don't()")
                .map(|(line, _)| line)
                .unwrap_or(line)
        })
        .flat_map(|l| l.split("mul("))
        .skip(1)
        .filter_map(parse_args)
        .sum()
}

fn parse_args(s: &str) -> Option<u64> {
    let (l, s) = s.split_once(',')?;
    let (r, _) = s.split_once(')')?;
    let l: u64 = l.parse().ok()?;
    let r: u64 = r.parse().ok()?;
    Some(l * r)
}
