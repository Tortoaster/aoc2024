use std::{cmp::Ordering, iter};

use cached::proc_macro::cached;

use crate::vec2::Vec2;

pub fn solve_a(input: &str) -> u64 {
    solve(input, 4)
}

pub fn solve_b(input: &str) -> u64 {
    solve(input, 27)
}

fn solve(input: &str, depth: usize) -> u64 {
    let (input, numbers) = parse_input(input);

    length(input, depth)
        .zip(numbers)
        .map(|(length, number)| length * number)
        .sum()
}

fn length(input: Vec<Vec<Button>>, depth: usize) -> impl Iterator<Item = u64> {
    input
        .into_iter()
        .map(|buttons| {
            iter::once(Button::Activate)
                .chain(buttons)
                .collect::<Vec<_>>()
        })
        .map(move |path| {
            path.windows(2)
                .map(|from_to| expand(from_to[0], from_to[1], depth - 1))
                .sum::<u64>()
        })
}

#[cached]
fn expand(from: Button, to: Button, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    length(from.paths_to(&to), depth).min().unwrap()
}

fn parse_input(input: &str) -> (Vec<Vec<Button>>, Vec<u64>) {
    input
        .lines()
        .map(|line| {
            (
                line.chars()
                    .map(|c| Button::from_char(c).unwrap())
                    .collect(),
                line.strip_suffix('A').unwrap().parse::<u64>().unwrap(),
            )
        })
        .unzip()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Button {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    Up,
    Left,
    Down,
    Right,

    Activate,
}

impl Button {
    const EMPTY: Vec2<isize> = Vec2::new(0, 1);

    fn from_char(c: char) -> Option<Self> {
        match c {
            '0' => Some(Self::Zero),
            '1' => Some(Self::One),
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            'A' => Some(Self::Activate),
            _ => None,
        }
    }

    // Both types of keypads are overlaid on top of each other. All buttons of the
    // numeric keypad are shifted upwards by 1, so that the activate button and the
    // empty space have the same position in both keypads. The zero and up button
    // share the same space as well, but buttons from the directional keypad are
    // never compared to buttons from the numeric keypad anyway.
    fn pos(&self) -> Vec2<isize> {
        match self {
            Button::Zero => Vec2::new(1, 1),
            Button::One => Vec2::new(0, 2),
            Button::Two => Vec2::new(1, 2),
            Button::Three => Vec2::new(2, 2),
            Button::Four => Vec2::new(0, 3),
            Button::Five => Vec2::new(1, 3),
            Button::Six => Vec2::new(2, 3),
            Button::Seven => Vec2::new(0, 4),
            Button::Eight => Vec2::new(1, 4),
            Button::Nine => Vec2::new(2, 4),
            Button::Up => Vec2::new(1, 1),
            Button::Left => Vec2::new(0, 0),
            Button::Down => Vec2::new(1, 0),
            Button::Right => Vec2::new(2, 0),
            Button::Activate => Vec2::new(2, 1),
        }
    }

    // The sequence of pressed buttons will never be shorter by pressing buttons
    // opposite to the side you need to go to, nor will it be more efficient to
    // alternate between the two directions that do go to the correct side. For any
    // combination of buttons, only at most 2 paths will need to be explored.
    fn paths_to(&self, dest: &Self) -> Vec<Vec<Button>> {
        let delta: Vec2<isize> = dest.pos() - self.pos();

        let mut horizontal = match delta.x.cmp(&0) {
            Ordering::Less => iter::repeat_n(Button::Left, delta.x.unsigned_abs()).collect(),
            Ordering::Equal => Vec::new(),
            Ordering::Greater => iter::repeat_n(Button::Right, delta.x as usize).collect(),
        };

        let mut vertical = match delta.y.cmp(&0) {
            Ordering::Less => iter::repeat_n(Button::Down, delta.y.unsigned_abs()).collect(),
            Ordering::Equal => Vec::new(),
            Ordering::Greater => iter::repeat_n(Button::Up, delta.y as usize).collect(),
        };

        if horizontal.is_empty() {
            vertical.push(Button::Activate);
            vec![vertical]
        } else if vertical.is_empty() {
            horizontal.push(Button::Activate);
            vec![horizontal]
        } else {
            let mut first_vertical = vertical.clone();
            first_vertical.append(&mut horizontal.clone());
            first_vertical.push(Button::Activate);
            let mut first_horizontal = horizontal;
            first_horizontal.append(&mut vertical);
            first_horizontal.push(Button::Activate);

            if self.pos().x == Self::EMPTY.x && dest.pos().y == Self::EMPTY.y {
                vec![first_horizontal]
            } else if dest.pos().x == Self::EMPTY.x && self.pos().y == Self::EMPTY.y {
                vec![first_vertical]
            } else {
                vec![first_horizontal, first_vertical]
            }
        }
    }
}
