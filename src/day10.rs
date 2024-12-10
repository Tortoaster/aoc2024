use std::collections::BTreeSet;

pub fn solve_a(input: &str) -> u64 {
    let map = parse_input(input);
    let trailheads = trailheads(&map);
    trailheads
        .into_iter()
        .map(|head| score(head, &map).len() as u64)
        .sum()
}

pub fn solve_b(input: &str) -> u64 {
    let map = parse_input(input);
    let trailheads = trailheads(&map);
    trailheads.into_iter().map(|head| rating(head, &map)).sum()
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.split("").filter_map(|c| c.parse().ok()).collect())
        .collect()
}

fn trailheads(map: &[Vec<usize>]) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, height)| (*height == 0).then_some((x, y)))
        })
        .collect()
}

fn score(pos: (usize, usize), map: &[Vec<usize>]) -> BTreeSet<(usize, usize)> {
    let height = map[pos.1][pos.0];
    if height == 9 {
        BTreeSet::from([pos])
    } else {
        next(height, pos, map)
            .flat_map(|pos| score(pos, map))
            .collect()
    }
}

fn rating(pos: (usize, usize), map: &[Vec<usize>]) -> u64 {
    let height = map[pos.1][pos.0];
    if height == 9 {
        1
    } else {
        next(height, pos, map).map(|pos| rating(pos, map)).sum()
    }
}

fn next(
    height: usize,
    pos: (usize, usize),
    map: &[Vec<usize>],
) -> impl Iterator<Item = (usize, usize)> + '_ {
    vec![
        (pos.0, pos.1.wrapping_sub(1)),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0.wrapping_sub(1), pos.1),
    ]
    .into_iter()
    .filter(|&(x, y)| map.get(y).and_then(|row| row.get(x)).is_some())
    .filter(move |&(x, y)| map[y][x] == height + 1)
}

#[cfg(test)]
mod tests {
    use super::{solve_a, solve_b};

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_a() {
        assert_eq!(solve_a(INPUT), 36);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(INPUT), 81);
    }
}
