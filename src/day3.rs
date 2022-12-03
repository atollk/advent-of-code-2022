use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn find_solution(inputs: &Vec<Vec<&str>>) -> Vec<i32> {
    let rucksacks = inputs.iter().map(|group| {
        group.iter().map(|x| x.chars().collect::<HashSet<_>>()).collect::<Vec<_>>()
    })
        .collect::<Vec<_>>();
    let bad_items = rucksacks
        .iter()
        .map(|xss| xss[0].iter().filter(move |x| xss[1..].iter().all(|ys| ys.contains(x))).collect::<HashSet<_>>())
        .map(|xs| *xs.iter().exactly_one().unwrap())
        .collect::<Vec<_>>();
    let scores = bad_items
        .iter()
        .map(|c| *c.to_string().as_bytes().into_iter().exactly_one().unwrap())
        .map(|c| if c.is_ascii_lowercase() { c - b'a' + 1 } else { c - b'A' + 27 })
        .collect::<Vec<_>>();
    scores.into_iter().map(|x| x as i32).collect()
}

pub fn day_3() {
    let file_contents = fs::read_to_string("day3_puzzle.txt").expect("Unable to read file");
    let file_lines = file_contents.split("\n").filter(|line| !line.is_empty()).collect::<Vec<&str>>();

    // part 1
    let rucksacks = file_lines.iter().map(|line| {
        let (a, b) = line.split_at(line.len() / 2);
        vec![a, b]
    })
        .collect::<Vec<_>>();
    let score = find_solution(&rucksacks).iter().map(|&x| x as u32).sum::<u32>();
    println!("{:?}", score);

    // part 2
    let groups = file_lines.chunks(3).map(|chunk| chunk.to_vec()).collect::<Vec<_>>();
    let score = find_solution(&groups).iter().map(|&x| x as u32).sum::<u32>();
    println!("{:?}", score);
}
