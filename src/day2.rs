use std::fs;

#[derive(Debug, Copy, Clone)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn wins_against(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Scissors,
            RockPaperScissors::Paper => RockPaperScissors::Rock,
            RockPaperScissors::Scissors => RockPaperScissors::Paper
        }
    }

    fn draws_against(&self) -> RockPaperScissors {
        *self
    }

    fn loses_against(&self) -> RockPaperScissors {
        match self {
            RockPaperScissors::Rock => RockPaperScissors::Paper,
            RockPaperScissors::Paper => RockPaperScissors::Scissors,
            RockPaperScissors::Scissors => RockPaperScissors::Rock
        }
    }
}

#[allow(dead_code)]
pub fn day_2() {
    let file_contents = fs::read_to_string("day2_puzzle.txt").expect("Unable to read file");
    let file_lines = file_contents.split("\n").collect::<Vec<&str>>();
    let game_rounds_1 = file_lines.iter()
        .filter(|&&line| !line.is_empty())
        .map(|&line| {
            let a = line.chars().nth(0).expect(line);
            let b = line.chars().nth(2).expect(line);
            let opp_sign = match a {
                'A' => RockPaperScissors::Rock,
                'B' => RockPaperScissors::Paper,
                'C' => RockPaperScissors::Scissors,
                _ => panic!()
            };
            let my_sign = match b {
                'X' => RockPaperScissors::Rock,
                'Y' => RockPaperScissors::Paper,
                'Z' => RockPaperScissors::Scissors,
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
                'A' => RockPaperScissors::Rock,
                'B' => RockPaperScissors::Paper,
                'C' => RockPaperScissors::Scissors,
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
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => 4,
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => 1,
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => 7,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => 8,
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => 5,
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => 2,
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => 3,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => 9,
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => 6,
        }
    }

    let scores_1 = game_rounds_1.iter().map(|(a, b)| round_score(a, b)).collect::<Vec<_>>();
    let scores_2 = game_rounds_2.iter().map(|(a, b)| round_score(a, b)).collect::<Vec<_>>();

    println!("{:?}", scores_1.iter().sum::<i32>());
    println!("{:?}", scores_2.iter().sum::<i32>());
}
