pub fn solve_a(input: &str) -> u64 {
    input
        .split("mul(")
        .skip(1)
        .filter_map(|s| {
            let (l, s) = s.split_once(',')?;
            let (r, _) = s.split_once(')')?;
            let l: u64 = l.parse().ok()?;
            let r: u64 = r.parse().ok()?;
            Some(l * r)
        })
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    input
        .split("don't()")
        .enumerate()
        .map(|(index, line)| {
            if index == 0 {
                line
            } else {
                line.split_once("do()").map(|(_, line)| line).unwrap_or("")
            }
        })
        .flat_map(|l| l.split("mul("))
        .skip(1)
        .filter_map(|s| {
            let (l, s) = s.split_once(',')?;
            let (r, _) = s.split_once(')')?;
            let l: u64 = l.parse().ok()?;
            let r: u64 = r.parse().ok()?;
            Some(l * r)
        })
        .sum()
}
