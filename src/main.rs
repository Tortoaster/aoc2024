use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    iter, thread,
    thread::JoinHandle,
    time::{Duration, Instant},
};

use arboard::Clipboard;
use crossterm::{
    event,
    event::{Event, KeyCode, KeyEvent},
};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Padding},
    Frame,
};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    #[rustfmt::skip]
    let days: Vec<(_, _, fn(_) -> _, fn(_) -> _)> = vec![
        (1, include_str!("../input/day1.txt"), day1::solve_a, day1::solve_b),
        (2, include_str!("../input/day2.txt"), day2::solve_a, day2::solve_b),
        (3, include_str!("../input/day3.txt"), day3::solve_a, day3::solve_b),
        (4, include_str!("../input/day4.txt"), day4::solve_a, day4::solve_b),
        (5, include_str!("../input/day5.txt"), day5::solve_a, day5::solve_b),
        (6, include_str!("../input/day6.txt"), day6::solve_a, day6::solve_b),
        (7, include_str!("../input/day7.txt"), day7::solve_a, day7::solve_b),
        (8, include_str!("../input/day8.txt"), day8::solve_a, day8::solve_b),
        (9, include_str!("../input/day9.txt"), day9::solve_a, day9::solve_b),
        (10, include_str!("../input/day10.txt"), day10::solve_a, day10::solve_b),
        (11, include_str!("../input/day11.txt"), day11::solve_a, day11::solve_b),
        (12, include_str!("../input/day12.txt"), day12::solve_a, day12::solve_b),
        // (13, include_str!("../input/day13.txt"), day13::solve_a, day13::solve_b),
        (14, include_str!("../input/day14.txt"), day14::solve_a, day14::solve_b),
    ];

    let mut outputs: BTreeMap<(i32, Part), _> = days
        .into_iter()
        .flat_map(|(day, input, solve_a, solve_b)| {
            [
                (day, Part::One, input, solve_a),
                (day, Part::Two, input, solve_b),
            ]
        })
        .map(|(day, part, input, solve)| {
            (
                (day, part),
                Remember::new(thread::spawn(move || solve(input))),
            )
        })
        .collect();
    let mut state = State::new(&outputs);

    let mut clipboard = Clipboard::new().unwrap();
    let mut terminal = ratatui::init();
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();
    loop {
        terminal
            .draw(|frame| draw(frame, &mut outputs, &state))
            .unwrap();

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
                match code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Left | KeyCode::Char('h') => state.move_left(),
                    KeyCode::Down | KeyCode::Char('j') => state.move_down(),
                    KeyCode::Up | KeyCode::Char('k') => state.move_up(),
                    KeyCode::Right | KeyCode::Char('l') => state.move_right(),
                    KeyCode::Enter => {
                        if let Some(&output) = outputs
                            .get_mut(&(state.day, state.part))
                            .and_then(Remember::poll)
                        {
                            clipboard.set_text(output.to_string()).unwrap();
                            state.copy();
                        }
                    }
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    ratatui::restore();
}

fn draw(frame: &mut Frame, outputs: &mut BTreeMap<(i32, Part), Remember<u64>>, state: &State) {
    for (row_index, row) in Layout::vertical(iter::repeat_n(Constraint::Length(6), 5))
        .split(frame.area())
        .iter()
        .enumerate()
    {
        for (col_index, tile) in Layout::horizontal(iter::repeat_n(
            Constraint::Length(u64::MAX.to_string().len() as u16 + 14),
            5,
        ))
        .split(*row)
        .iter()
        .enumerate()
        {
            let day = (row_index * 5 + col_index + 1) as i32;

            let mut block = Block::bordered()
                .border_type(BorderType::Rounded)
                .title_alignment(Alignment::Center)
                .padding(Padding::proportional(1))
                .title(format!("[  Day {day}  ]"));
            block = if state.day == day {
                block.light_yellow()
            } else {
                block.gray()
            };
            let block_inner = block.inner(*tile);
            frame.render_widget(block, *tile);

            for (part, line) in Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
                .split(block_inner)
                .iter()
                .enumerate()
            {
                let part = Part::try_from(part + 1).unwrap();

                let chunks = Layout::horizontal([
                    Constraint::Length(8),
                    Constraint::Length(u64::MAX.to_string().len() as u16),
                ])
                .split(*line);

                let mut label = match state.copied {
                    Some((copy_day, copy_part, time))
                        if copy_day == day
                            && copy_part == part
                            && time.elapsed() < Duration::from_secs(1) =>
                    {
                        Span::from("Copied! ")
                    }
                    _ => Span::from(format!("Part {}: ", part)),
                };
                label = if state.day == day && state.part == part {
                    label.light_yellow()
                } else {
                    label.white()
                };
                frame.render_widget(label, chunks[0]);

                let output = match outputs.get_mut(&(day, part)) {
                    None => Span::from("-").gray(),
                    Some(handle) => match handle.poll() {
                        None => Span::from("...").gray(),
                        Some(output) => Span::from(output.to_string()).cyan(),
                    },
                }
                .into_right_aligned_line();

                frame.render_widget(output, chunks[1]);
            }
        }
    }
}

#[derive(Debug)]
struct State {
    day: i32,
    part: Part,
    copied: Option<(i32, Part, Instant)>,
}

impl State {
    pub fn new(outputs: &BTreeMap<(i32, Part), Remember<u64>>) -> Self {
        let (day, part) = outputs.keys().max().copied().unwrap_or((1, Part::One));
        Self {
            day,
            part,
            copied: None,
        }
    }

    fn copy(&mut self) {
        self.copied = Some((self.day, self.part, Instant::now()));
    }

    fn move_left(&mut self) {
        self.day = (self.day - 1) / 5 * 5 + ((self.day - 1) % 5 - 1).rem_euclid(5) + 1;
    }

    fn move_right(&mut self) {
        self.day = (self.day - 1) / 5 * 5 + ((self.day - 1) % 5 + 1) % 5 + 1;
    }

    fn move_up(&mut self) {
        if self.part == Part::Two {
            self.part = Part::One;
        } else {
            self.day = ((self.day - 1) / 5 - 1).rem_euclid(5) * 5 + (self.day - 1) % 5 + 1;
            self.part = Part::Two;
        }
    }

    fn move_down(&mut self) {
        if self.part == Part::One {
            self.part = Part::Two;
        } else {
            self.day = ((self.day - 1) / 5 + 1) % 5 * 5 + (self.day - 1) % 5 + 1;
            self.part = Part::One;
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Part {
    One,
    Two,
}

impl TryFrom<usize> for Part {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err("invalid part"),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

pub struct Remember<T> {
    handle: Option<JoinHandle<T>>,
    output: Option<T>,
}

impl<T> Remember<T> {
    pub fn new(handle: JoinHandle<T>) -> Self {
        Self {
            handle: Some(handle),
            output: None,
        }
    }

    pub fn poll(&mut self) -> Option<&T> {
        if self.output.is_some() {
            self.output.as_ref()
        } else if self.handle.as_ref()?.is_finished() {
            self.output = Some(self.handle.take()?.join().unwrap());
            self.output.as_ref()
        } else {
            None
        }
    }
}
