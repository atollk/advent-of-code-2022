use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use std::iter::repeat;
use itertools::Itertools;


#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_string(string: &str) -> Option<Direction> {
        let c = string.chars().exactly_one().ok()?;
        match c {
            'D' => Some(Direction::Down),
            'U' => Some(Direction::Up),
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Move {
    direction: Direction,
    distance: usize,
}

fn read_input() -> Vec<Move> {
    let file_contents = fs::read_to_string("day9_puzzle.txt").expect("Unable to read file");
    file_contents.split("\n").map(|line| {
        let (a, b) = line.split_once(" ").unwrap();
        Move { direction: Direction::from_string(a).unwrap(), distance: b.parse().unwrap() }
    }).collect_vec()
}

#[derive(Debug, Clone)]
struct RopeState {
    head: (i32, i32),
    tail: (i32, i32),
    visited_tail_positions: HashSet<(i32, i32)>,
}

impl RopeState {
    fn new() -> RopeState {
        RopeState {
            head: (0, 0),
            tail: (0, 0),
            visited_tail_positions: HashSet::from([(0, 0)]),
        }
    }

    fn apply_move(&mut self, mov: &Move) {
        assert_eq!(mov.distance, 1);

        // Move head.
        match mov.direction {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Left => self.head.0 -= 1,
            Direction::Right => self.head.0 += 1,
        }

        // Move tail.
        let delta = (self.head.0 - self.tail.0, self.head.1 - self.tail.1);
        if delta.0 >= 2 {
            self.tail.0 += 1;
            self.tail.1 += num::signum(delta.1);
        } else if delta.0 <= -2 {
            self.tail.0 -= 1;
            self.tail.1 += num::signum(delta.1);
        } else if delta.1 >= 2 {
            self.tail.1 += 1;
            self.tail.0 += num::signum(delta.0);
        } else if delta.1 <= -2 {
            self.tail.1 -= 1;
            self.tail.0 += num::signum(delta.0);
        }

        // Record tail position.
        self.visited_tail_positions.insert(self.tail);
    }

    fn pretty_print(&self) {
        let min_x = min(self.visited_tail_positions.iter().map(|p| p.0).min().unwrap(), self.head.0);
        let max_x = max(self.visited_tail_positions.iter().map(|p| p.0).max().unwrap(), self.head.0);
        let min_y = min(self.visited_tail_positions.iter().map(|p| p.1).min().unwrap(), self.head.1);
        let max_y = max(self.visited_tail_positions.iter().map(|p| p.1).max().unwrap(), self.head.1);
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let c = if self.head == (x, y) {
                    "H"
                } else if self.tail == (x, y) {
                    "T"
                } else if (0, 0) == (x, y) {
                    "s"
                } else if self.visited_tail_positions.contains(&(x, y)) {
                    "#"
                } else {
                    "."
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

#[allow(dead_code)]
pub fn day_9() {
    let move_list = read_input();

    let simplified_move_list = move_list.iter().flat_map(|mov| {
        repeat(Move { direction: mov.direction.clone(), distance: 1 }).take(mov.distance)
    }).collect_vec();

    let mut state = RopeState::new();

    for mov in simplified_move_list {
        state.apply_move(&mov);
    }

    println!("{:?}", state.visited_tail_positions.len());
}