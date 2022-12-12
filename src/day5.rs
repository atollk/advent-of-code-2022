use itertools::Itertools;
use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Day5Input {
    initial_stacks: Stacks,
    move_list: Vec<StackMove>,
}

impl Day5Input {
    fn from_input(file_contents: &str) -> Day5Input {
        let (file_contents_1, file_contents_2) = file_contents.split_once("\n\n").unwrap();

        let initial_stacks = {
            let mut lines = file_contents_1.split("\n").collect::<Vec<&str>>();
            let last_line = *lines.last().unwrap();
            lines.truncate(lines.len() - 1);
            let number_of_stacks = last_line.split_ascii_whitespace().count();
            let mut result = vec![Vec::new(); number_of_stacks];
            for i in 0..result.len() {
                let mut stack = Vec::new();
                for l in lines.iter() {
                    let c = l.chars().nth(1 + 4 * i).unwrap().clone();
                    if c.is_alphabetic() {
                        stack.push(c);
                    }
                }
                result[i] = stack.into_iter().rev().collect::<Vec<_>>();
            }
            result
        };

        let move_list = {
            let regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
            file_contents_2
                .split("\n")
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let captures = regex.captures(line).unwrap();
                    assert_eq!(captures.len(), 4);
                    let (c1, c2, c3) = (&captures[1], &captures[2], &captures[3]);
                    let crates = c1.parse::<usize>().unwrap();
                    let from = c2.parse::<usize>().unwrap();
                    let to = c3.parse::<usize>().unwrap();
                    StackMove { from, to, crates }
                })
                .collect::<Vec<_>>()
        };

        Day5Input {
            initial_stacks: Stacks(initial_stacks),
            move_list,
        }
    }
}

#[derive(Debug, Clone)]
struct Stacks(Vec<Vec<char>>);

impl Stacks {
    fn apply_move_9000(&mut self, stack_move: &StackMove) {
        for _ in 0..stack_move.crates {
            let x = self.0[stack_move.from - 1].pop().unwrap();
            self.0[stack_move.to - 1].push(x);
        }
    }

    fn apply_move_9001(&mut self, stack_move: &StackMove) {
        let from_stack = &mut self.0[stack_move.from - 1];
        let mut moved_elements = from_stack
            .drain(from_stack.len() - stack_move.crates..)
            .collect_vec();
        self.0[stack_move.to - 1].append(&mut moved_elements);
    }
}

#[derive(Debug)]
struct StackMove {
    from: usize,
    to: usize,
    crates: usize,
}

#[allow(dead_code)]
pub fn day_5() {
    let file_contents = fs::read_to_string("day5_puzzle.txt").expect("Unable to read file");
    let input = Day5Input::from_input(&file_contents);

    // Star 1
    let mut stacks = input.initial_stacks.clone();
    for stack_move in input.move_list.iter() {
        stacks.apply_move_9000(stack_move);
    }
    let top_elements = stacks.0.iter().map(|stack| stack.last().unwrap()).join("");
    println!("{:?}", top_elements);

    // Star 2
    let mut stacks = input.initial_stacks.clone();
    for stack_move in input.move_list.iter() {
        stacks.apply_move_9001(stack_move);
    }
    let top_elements = stacks.0.iter().map(|stack| stack.last().unwrap()).join("");
    println!("{:?}", top_elements);
}
