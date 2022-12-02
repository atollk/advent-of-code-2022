use std::fs;
use crate::RockPaperScissors::{Paper, Rock, Scissors};

fn day_1() {
    let file_contents = fs::read_to_string("day1_puzzle.txt").expect("Unable to read file");
    let file_lines = file_contents.split("\n").collect::<Vec<&str>>();
    let mut elf_food = vec![Vec::new()];

    for line in file_lines {
        if line.is_empty() {
            elf_food.push(Vec::new());
        } else {
            let n: i32 = line.parse().expect(&format!("{:?}", line));
            elf_food.last_mut().unwrap().push(n);
        }
    }

    let mut elf_food_sum = elf_food.iter().map(|xs| xs.iter().sum::<i32>()).collect::<Vec<i32>>();
    elf_food_sum.sort();

    println!("{:?}", elf_food_sum);
    println!("star1: {:?}", elf_food_sum.iter().max());
    println!("star2: {:?}", elf_food_sum.iter().rev().take(3).sum::<i32>());
}

#[derive(Debug, Copy, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn wins_against(&self) -> RockPaperScissors {
        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        }
    }

    fn draws_against(&self) -> RockPaperScissors {
        *self
    }

    fn loses_against(&self) -> RockPaperScissors {
        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }
    }
}

fn day_2() {
    let file_contents = fs::read_to_string("day2_puzzle.txt").expect("Unable to read file");
    let file_lines = file_contents.split("\n").collect::<Vec<&str>>();
    let game_rounds_1 = file_lines.iter()
        .filter(|&&line| !line.is_empty())
        .map(|&line| {
            let a = line.chars().nth(0).expect(line);
            let b = line.chars().nth(2).expect(line);
            let opp_sign = match a {
                'A' => Rock,
                'B' => Paper,
                'C' => Scissors,
                _ => panic!()
            };
            let my_sign = match b {
                'X' => Rock,
                'Y' => Paper,
                'Z' => Scissors,
                _ => panic!()
            };
            (my_sign, opp_sign)
        }).collect::<Vec<_>>();
    let game_rounds_2 = file_lines.iter()
        .filter(|&&line| !line.is_empty())
        .map(|&line| {
            let a = line.chars().nth(0).expect(line);
            let b = line.chars().nth(2).expect(line);
            let opp_sign = match a {
                'A' => Rock,
                'B' => Paper,
                'C' => Scissors,
                _ => panic!()
            };
            let my_sign = match b {
                'X' => opp_sign.wins_against(),
                'Y' => opp_sign.draws_against(),
                'Z' => opp_sign.loses_against(),
                _ => panic!()
            };
            (my_sign, opp_sign)
        }).collect::<Vec<_>>();

    fn round_score(me: &RockPaperScissors, opp: &RockPaperScissors) -> i32 {
        match (me, opp) {
            (Rock, Rock) => 4,
            (Rock, Paper) => 1,
            (Rock, Scissors) => 7,
            (Paper, Rock) => 8,
            (Paper, Paper) => 5,
            (Paper, Scissors) => 2,
            (Scissors, Rock) => 3,
            (Scissors, Paper) => 9,
            (Scissors, Scissors) => 6,
        }
    }

    let scores_1 = game_rounds_1.iter().map(|(a, b)| round_score(a, b)).collect::<Vec<_>>();
    let scores_2 = game_rounds_2.iter().map(|(a, b)| round_score(a, b)).collect::<Vec<_>>();

    println!("{:?}", scores_1.iter().sum::<i32>());
    println!("{:?}", scores_2.iter().sum::<i32>());
}

fn main() {
    day_2();
}
