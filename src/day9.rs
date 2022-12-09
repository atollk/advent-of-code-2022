use std::collections::HashSet;
use std::{fs, iter};
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

fn read_input() -> Vec<Direction> {
    let file_contents = fs::read_to_string("day9_puzzle.txt").expect("Unable to read file");
    file_contents.split("\n").flat_map(|line| {
        let (a, b) = line.split_once(" ").unwrap();
        iter::repeat(Direction::from_string(a).unwrap()).take(b.parse().unwrap())
    }).collect_vec()
}

#[derive(Debug, Clone)]
struct RopeState {
    knots: Vec<(i32, i32)>,
    visited_tail_positions: HashSet<(i32, i32)>,
}

impl RopeState {
    fn new(knot_count: usize) -> RopeState {
        RopeState {
            knots: vec![(0, 0); knot_count],
            visited_tail_positions: HashSet::from([(0, 0)]),
        }
    }

    fn apply_move(&mut self, direction: &Direction) {
        // Move head.
        {
            let head = self.knots.first_mut().unwrap();
            match direction {
                Direction::Up => head.1 += 1,
                Direction::Down => head.1 -= 1,
                Direction::Left => head.0 -= 1,
                Direction::Right => head.0 += 1,
            }
        }

        // Move following knots.
        {
            for leading_i in 0..self.knots.len() - 1 {
                let leading = self.knots[leading_i];
                let following = self.knots.get_mut(leading_i + 1).unwrap();
                let delta = (leading.0 - following.0, leading.1 - following.1);
                if delta.0 >= 2 {
                    following.0 += 1;
                    following.1 += num::signum(delta.1);
                } else if delta.0 <= -2 {
                    following.0 -= 1;
                    following.1 += num::signum(delta.1);
                } else if delta.1 >= 2 {
                    following.1 += 1;
                    following.0 += num::signum(delta.0);
                } else if delta.1 <= -2 {
                    following.1 -= 1;
                    following.0 += num::signum(delta.0);
                }
            }
        }

        // Record tail position.
        self.visited_tail_positions.insert(*self.knots.last().unwrap());
    }

    #[allow(dead_code)]
    fn pretty_print(&self) {
        let x_positions = self.visited_tail_positions.iter().map(|p| p.0).chain(self.knots.iter().map(|knot| knot.0));
        let y_positions = self.visited_tail_positions.iter().map(|p| p.1).chain(self.knots.iter().map(|knot| knot.1));
        let min_x = x_positions.clone().min().unwrap();
        let max_x = x_positions.clone().max().unwrap();
        let min_y = y_positions.clone().min().unwrap();
        let max_y = y_positions.clone().max().unwrap();
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let c = if let Some((knot_i, _)) = self.knots.iter().find_position(|knot| **knot == (x, y)) {
                    knot_i.to_string()
                } else {
                    if (0, 0) == (x, y) {
                        "s"
                    } else if self.visited_tail_positions.contains(&(x, y)) {
                        "#"
                    } else {
                        "."
                    }.to_string()
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

fn solve_with_knots(knot_count: usize, move_list: &Vec<Direction>) -> usize {
    let mut state = RopeState::new(knot_count);

    for mov in move_list {
        state.apply_move(&mov);
    }

    state.visited_tail_positions.len()
}

#[allow(dead_code)]
pub fn day_9() {
    let move_list = read_input();

    println!("star 1 : {}", solve_with_knots(2, &move_list));
    println!("star 2 : {}", solve_with_knots(10, &move_list));
}