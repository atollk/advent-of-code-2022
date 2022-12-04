
use std::fs;

#[allow(dead_code)]
pub fn day_1() {
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