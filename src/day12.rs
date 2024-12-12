use disjoint_hash_set::DisjointHashSet;
use std::collections::BTreeSet;
use std::iter;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

pub fn solve_a(input: &str) -> u64 {
    let (_, _, sides, regions) = parse_input(input);

    regions
        .sets()
        .map(|set| set.len() * set.iter().map(|&(x, y)| sides[y][x].len()).sum::<usize>())
        .sum::<usize>() as u64
}

pub fn solve_b(input: &str) -> u64 {
    let (width, height, sides, regions) = parse_input(input);

    regions
        .sets()
        .map(|set| {
            let mut top_sides = DisjointHashSet::new();
            let mut right_sides = DisjointHashSet::new();
            let mut bottom_sides = DisjointHashSet::new();
            let mut left_sides = DisjointHashSet::new();

            for &(x, y) in &set {
                let sides = &sides[y][x];
                if sides.contains(&Side::Top) {
                    top_sides.insert((x, y));
                }
                if sides.contains(&Side::Right) {
                    right_sides.insert((x, y));
                }
                if sides.contains(&Side::Bottom) {
                    bottom_sides.insert((x, y));
                }
                if sides.contains(&Side::Left) {
                    left_sides.insert((x, y));
                }
            }

            for y in 0..height - 1 {
                for x in 0..width {
                    if left_sides.contains((x, y)) && left_sides.contains((x, y + 1)) {
                        left_sides.link((x, y), (x, y + 1))
                    }
                    if right_sides.contains((x, y)) && right_sides.contains((x, y + 1)) {
                        right_sides.link((x, y), (x, y + 1))
                    }
                }
            }
            for y in 0..height {
                for x in 0..width - 1 {
                    if top_sides.contains((x, y)) && top_sides.contains((x + 1, y)) {
                        top_sides.link((x, y), (x + 1, y))
                    }
                    if bottom_sides.contains((x, y)) && bottom_sides.contains((x + 1, y)) {
                        bottom_sides.link((x, y), (x + 1, y))
                    }
                }
            }

            set.len()
                * (top_sides.sets().count()
                    + right_sides.sets().count()
                    + bottom_sides.sets().count()
                    + left_sides.sets().count())
        })
        .sum::<usize>() as u64
}

fn parse_input(
    input: &str,
) -> (
    usize,
    usize,
    Vec<Vec<BTreeSet<Side>>>,
    DisjointHashSet<(usize, usize)>,
) {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let width = map[0].len();
    let height = map.len();

    let empty_row: Vec<_> = iter::repeat('-').take(width).collect();
    let shift_up = map[1..].iter().chain(iter::once(&empty_row));
    let shift_right = map
        .iter()
        .map(|row| iter::once(&'-').chain(&row[..row.len() - 1]));
    let shift_down = iter::once(&empty_row).chain(&map[..height - 1]);
    let shift_left = map
        .iter()
        .map(|row| row[1..].iter().chain(iter::once(&'-')));

    let sides: Vec<Vec<_>> = map
        .iter()
        .zip(shift_up)
        .zip(shift_right)
        .zip(shift_down)
        .zip(shift_left)
        .map(|((((row, up), right), down), left)| {
            row.iter()
                .zip(up)
                .zip(right)
                .zip(down)
                .zip(left)
                .map(|((((c, up), right), down), left)| {
                    let mut sides = BTreeSet::new();
                    if c != up {
                        sides.insert(Side::Top);
                    }
                    if c != right {
                        sides.insert(Side::Right);
                    }
                    if c != down {
                        sides.insert(Side::Bottom);
                    }
                    if c != left {
                        sides.insert(Side::Left);
                    }
                    sides
                })
                .collect()
        })
        .collect();

    let mut regions = DisjointHashSet::new();
    for (y, row) in map.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            regions.insert((x, y));
        }
    }
    for y in 0..height - 1 {
        for x in 0..width {
            if map[y][x] == map[y + 1][x] {
                regions.link((x, y), (x, y + 1));
            }
        }
    }
    for y in 0..height {
        for x in 0..width - 1 {
            if map[y][x] == map[y][x + 1] {
                regions.link((x, y), (x + 1, y));
            }
        }
    }

    (width, height, sides, regions)
}

#[cfg(test)]
mod tests {
    use super::{solve_a, solve_b};

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn test_a() {
        assert_eq!(solve_a(INPUT), 1930);
    }

    #[test]
    fn test_b() {
        assert_eq!(solve_b(INPUT), 1206);
    }
}
