use std::fs;
use itertools::Itertools;
use num::abs;


#[derive(Debug, Clone)]
enum Operation {
    Noop,
    Addx(i32),
}

fn read_input() -> Vec<Operation> {
    let file_contents = fs::read_to_string("day10_puzzle.txt").expect("Unable to read file");
    file_contents.split("\n").map(|line| {
        if line == "noop" {
            Operation::Noop
        } else {
            let x = line.split_once(" ").unwrap().1.parse().unwrap();
            Operation::Addx(x)
        }
    }).collect_vec()
}

#[allow(dead_code)]
pub fn day_10() {
    let op_list = read_input();

    let register_values_by_cycle = {
        let cycle_additions = op_list
            .iter()
            .flat_map(|operation| {
                match operation {
                    Operation::Noop => vec![0],
                    Operation::Addx(x) => vec![0, *x]
                }
            })
            .collect_vec();
        let mut cycle_additions_cumsum = cycle_additions
            .iter()
            .scan(1, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect_vec();
        cycle_additions_cumsum.insert(0, 0);
        cycle_additions_cumsum.insert(1, 1);
        cycle_additions_cumsum
    };

    // Star 1
    {
        let relevant_values = vec![20usize, 60, 100, 140, 180, 220]
            .iter()
            .map(|&i| i as i32 * register_values_by_cycle[i])
            .collect_vec();
        println!("star 1: {}", relevant_values.iter().sum::<i32>());
    }

    // Star 2
    {
        for line in 0..6 {
            for x in 0..39 {
                let cycle = line * 40 + x + 1;
                let draw_sprite = abs(x as i32 - register_values_by_cycle[cycle]) < 2;
                if draw_sprite {
                    print!("##");
                } else {
                    print!("..");
                }
            }
            println!();
        }
    }
}