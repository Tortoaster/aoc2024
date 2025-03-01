use std::collections::BTreeSet;

use rayon::prelude::*;

#[derive(Clone, PartialEq)]
enum Tile {
    Guard,
    Obstacle,
    Ground,
    Visited,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Ground,
            '#' => Tile::Obstacle,
            '^' => Tile::Guard,
            _ => panic!("weird character"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn solve_a(input: &str) -> u64 {
    let (mut map, (x, y)) = parse_input(input);

    simulate(x, y, &mut map).unwrap();

    map.into_iter()
        .flatten()
        .filter(|tile| *tile == Tile::Visited)
        .count() as u64
}

pub fn solve_b(input: &str) -> u64 {
    let (map, (x_start, y_start)) = parse_input(input);

    (0..map.len())
        .into_par_iter()
        .flat_map(|y| (0..map[0].len()).into_par_iter().map(move |x| (x, y)))
        .filter(|&(x, y)| map[x][y] == Tile::Ground)
        .filter(|&(x, y)| {
            let mut map = map.clone();
            map[x][y] = Tile::Obstacle;
            simulate(x_start, y_start, &mut map).is_err()
        })
        .count() as u64
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, (usize, usize)) {
    let map: Vec<_> = input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect::<Vec<_>>())
        .collect();

    let start_pos = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|t| *t == Tile::Guard).map(|x| (x, y)))
        .unwrap();

    (map, start_pos)
}

fn simulate(mut x: usize, mut y: usize, map: &mut [Vec<Tile>]) -> Result<(), &str> {
    let mut dir = Direction::Up;
    let mut visited: Vec<_> = map
        .iter()
        .map(|row| row.iter().map(|_| BTreeSet::new()).collect::<Vec<_>>())
        .collect();
    visited[y][x].insert(dir);

    loop {
        let offset = dir.offset();
        let next_x = (x as isize + offset.0) as usize;
        let next_y = (y as isize + offset.1) as usize;
        match map.get_mut(next_y).and_then(|row| row.get_mut(next_x)) {
            None => {
                map[y][x] = Tile::Visited;
                return Ok(());
            }
            Some(tile) => match tile {
                Tile::Obstacle => dir = dir.rotate_right(),
                Tile::Ground | Tile::Visited => {
                    if *tile == Tile::Visited && visited[next_y][next_x].contains(&dir) {
                        return Err("stuck in a loop");
                    }
                    *tile = Tile::Guard;
                    map[y][x] = Tile::Visited;
                    x = next_x;
                    y = next_y;
                    visited[y][x].insert(dir);
                }
                Tile::Guard => panic!("spiderman meme"),
            },
        }
    }
}
