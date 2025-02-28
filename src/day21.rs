use std::iter;

use cached::proc_macro::cached;

pub fn solve_a(input: &str) -> u64 {
    solve(input, 3)
}

pub fn solve_b(input: &str) -> u64 {
    solve(input, 26)
}

fn solve(input: &str, depth: usize) -> u64 {
    let (input, numbers) = parse_input(input);

    input
        .into_iter()
        .map(|buttons| {
            iter::once(Button::Activate)
                .chain(buttons)
                .collect::<Vec<_>>()
        })
        .map(|path| {
            path.windows(2)
                .map(|from_to| expand_first(from_to[0], from_to[1], depth))
                .sum::<u64>()
        })
        .zip(numbers)
        .map(|(length, number)| length * number)
        .sum()
}

#[cached]
fn expand_first(from: Button, to: Button, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    from.paths_to(&to)
        .into_iter()
        .map(|buttons| {
            iter::once(DirButton::Activate)
                .chain(buttons)
                .collect::<Vec<_>>()
        })
        .map(|path| {
            path.windows(2)
                .map(|from_to| expand(from_to[0], from_to[1], depth - 1))
                .sum::<u64>()
        })
        .min()
        .unwrap()
}

#[cached]
fn expand(from: DirButton, to: DirButton, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    from.paths_to(&to)
        .into_iter()
        .map(|buttons| {
            iter::once(DirButton::Activate)
                .chain(buttons)
                .collect::<Vec<_>>()
        })
        .map(|path| {
            path.windows(2)
                .map(|from_to| expand(from_to[0], from_to[1], depth - 1))
                .sum::<u64>()
        })
        .min()
        .unwrap()
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
    Activate,
}

impl Button {
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
}

impl Pos for Button {
    const EMPTY_X: usize = 0;
    const EMPTY_Y: usize = 0;

    fn x(&self) -> usize {
        match self {
            Button::Zero => 1,
            Button::One => 0,
            Button::Two => 1,
            Button::Three => 2,
            Button::Four => 0,
            Button::Five => 1,
            Button::Six => 2,
            Button::Seven => 0,
            Button::Eight => 1,
            Button::Nine => 2,
            Button::Activate => 2,
        }
    }

    fn y(&self) -> usize {
        match self {
            Button::Zero => 0,
            Button::One => 1,
            Button::Two => 1,
            Button::Three => 1,
            Button::Four => 2,
            Button::Five => 2,
            Button::Six => 2,
            Button::Seven => 3,
            Button::Eight => 3,
            Button::Nine => 3,
            Button::Activate => 0,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum DirButton {
    Up,
    Left,
    Down,
    Right,
    Activate,
}

impl Pos for DirButton {
    const EMPTY_X: usize = 0;
    const EMPTY_Y: usize = 1;

    fn x(&self) -> usize {
        match self {
            DirButton::Up => 1,
            DirButton::Left => 0,
            DirButton::Down => 1,
            DirButton::Right => 2,
            DirButton::Activate => 2,
        }
    }

    fn y(&self) -> usize {
        match self {
            DirButton::Up => 1,
            DirButton::Left => 0,
            DirButton::Down => 0,
            DirButton::Right => 0,
            DirButton::Activate => 1,
        }
    }
}

trait Pos {
    const EMPTY_X: usize;
    const EMPTY_Y: usize;

    fn x(&self) -> usize;
    fn y(&self) -> usize;

    // The sequence of pressed buttons will never be shorter by pressing buttons
    // opposite to the side you need to go to, nor will it be more efficient to
    // alternate between the two directions that do go to the correct side. For any
    // combination of buttons, only at most 2 paths will need to be explored.
    fn paths_to(&self, dest: &Self) -> Vec<Vec<DirButton>> {
        let dx = dest.x() as isize - self.x() as isize;
        let dy = dest.y() as isize - self.y() as isize;

        let mut horizontal = if dx < 0 {
            iter::repeat_n(DirButton::Left, dx.abs() as usize).collect()
        } else if dx > 0 {
            iter::repeat_n(DirButton::Right, dx as usize).collect()
        } else {
            Vec::new()
        };

        let mut vertical = if dy < 0 {
            iter::repeat_n(DirButton::Down, dy.abs() as usize).collect()
        } else if dy > 0 {
            iter::repeat_n(DirButton::Up, dy as usize).collect()
        } else {
            Vec::new()
        };

        if horizontal.is_empty() {
            vertical.push(DirButton::Activate);
            vec![vertical]
        } else if vertical.is_empty() {
            horizontal.push(DirButton::Activate);
            vec![horizontal]
        } else {
            let first_vertical = vertical
                .iter()
                .copied()
                .chain(horizontal.clone())
                .chain(iter::once(DirButton::Activate))
                .collect();
            horizontal.append(&mut vertical);
            horizontal.push(DirButton::Activate);
            let first_horizontal = horizontal;

            if self.x() == Self::EMPTY_X && dest.y() == Self::EMPTY_Y {
                vec![first_horizontal]
            } else if dest.x() == Self::EMPTY_X && self.y() == Self::EMPTY_Y {
                vec![first_vertical]
            } else {
                vec![first_horizontal, first_vertical]
            }
        }
    }
}
