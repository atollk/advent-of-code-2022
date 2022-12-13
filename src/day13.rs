use itertools::Itertools;
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Signal {
    Integer(i32),
    List(Vec<Signal>),
}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Signal::Integer(x) => match other {
                Signal::Integer(y) => x.cmp(y),
                Signal::List(_) => Signal::List(vec![Signal::Integer(*x)]).cmp(other),
            },
            Signal::List(xs) => match other {
                Signal::Integer(y) => self.cmp(&Signal::List(vec![Signal::Integer(*y)])),
                Signal::List(ys) => {
                    for (x, y) in xs.iter().zip(ys) {
                        let cmp = x.cmp(y);
                        if cmp != Ordering::Equal {
                            return cmp;
                        }
                    }
                    xs.len().cmp(&ys.len())
                }
            },
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Signal {
    fn from_line(line: &str) -> Signal {
        assert_ne!(line.len(), 0);
        if line.chars().nth(0).unwrap() == '[' {
            let mut substrings = Vec::new();
            let mut depth = 0;
            let mut previous_index = 0;
            for (i, c) in line.char_indices() {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                    if depth == 0 {
                        let ss = &line[previous_index + 1..i];
                        if !ss.is_empty() {
                            substrings.push(ss);
                        }
                    }
                } else if c == ',' && depth == 1 {
                    substrings.push(&line[previous_index + 1..i]);
                    previous_index = i;
                }
            }
            Signal::List(
                substrings
                    .into_iter()
                    .map(|ss| Signal::from_line(ss))
                    .collect_vec(),
            )
        } else {
            Signal::Integer(line.parse().unwrap())
        }
    }
}

fn read_input() -> Vec<(Signal, Signal)> {
    let file_contents = fs::read_to_string("day13_puzzle.txt").expect("Unable to read file");
    file_contents
        .split("\n")
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            (
                Signal::from_line(chunk.next().unwrap()),
                Signal::from_line(chunk.next().unwrap()),
            )
        })
        .collect_vec()
}

#[allow(dead_code)]
pub fn day_13() {
    let input = read_input();

    println!("{:?}", input);

    println!(
        "star 1: {}",
        input
            .iter()
            .map(|(a, b)| a.cmp(b))
            .enumerate()
            .filter(|(_, ord)| *ord == Ordering::Less)
            .map(|(i, _)| i + 1)
            .sum::<usize>()
    );

    {
        let divider1 = Signal::List(vec![Signal::List(vec![Signal::Integer(2)])]);
        let divider2 = Signal::List(vec![Signal::List(vec![Signal::Integer(6)])]);
        let sorted_signals = input
            .iter()
            .flat_map(|(a, b)| vec![a, b])
            .chain(vec![&divider1, &divider2].into_iter())
            .sorted()
            .collect_vec();
        let divider1_index = sorted_signals
            .iter()
            .find_position(|&&s| *s == divider1)
            .unwrap()
            .0;
        let divider2_index = sorted_signals
            .iter()
            .find_position(|&&s| *s == divider2)
            .unwrap()
            .0;
        println!("star 2: {:?}", (divider1_index + 1) * (divider2_index + 1),);
    }
}
