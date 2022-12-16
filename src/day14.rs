use std::cmp::{max, min};
use itertools::Itertools;
use std::collections::{HashMap};
use std::fs;
use num::signum;

const X_SPAWN: i32 = 500;
const Y_SPAWN: i32 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Material {
    Rock,
    Sand,
}

#[derive(Debug, Clone)]
struct RockMap {
    x_range: (i32, i32),
    max_y: i32,
    material: HashMap<(i32, i32), Material>,
}

impl RockMap {
    fn pretty_print(&self) {
        for y in 0..=self.max_y {
            for x in self.x_range.0..=self.x_range.1 {
                let c = match self.material.get(&(x, y)) {
                    None => { '.' }
                    Some(m) => {
                        match m {
                            Material::Rock => { '#' }
                            Material::Sand => { 'o' }
                        }
                    }
                };
                print!("{} ", c);
            }
            println!();
        }
    }

    fn drop_sand_towards_void(&mut self, drop_start: (i32, i32)) -> Option<(i32, i32)> {
        let mut sand_pos = drop_start;
        while sand_pos.1 <= self.max_y {
            if !self.material.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
                sand_pos.1 += 1;
            } else if !self.material.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if !self.material.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                self.material.insert(sand_pos, Material::Sand);
                return Some(sand_pos);
            }
        }
        None
    }

    fn drop_sand_towards_floor(&mut self, drop_start: (i32, i32)) -> (i32, i32) {
        let mut sand_pos = drop_start;
        while sand_pos.1 < self.max_y {
            if !self.material.contains_key(&(sand_pos.0, sand_pos.1 + 1)) {
                sand_pos.1 += 1;
            } else if !self.material.contains_key(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if !self.material.contains_key(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                break;
            }
        }
        self.x_range.0 = min(self.x_range.0, sand_pos.0);
        self.x_range.1 = max(self.x_range.1, sand_pos.0);
        self.material.insert(sand_pos, Material::Sand);
        sand_pos
    }

    fn count_sand(&self) -> usize {
        self.material.values().filter(|m| **m == Material::Sand).count()
    }
}

fn read_input() -> Vec<Vec<(i32, i32)>> {
    let file_contents = fs::read_to_string("day14_puzzle.txt").expect("Unable to read file");
    file_contents
        .split("\n")
        .map(|line| {
            line
                .split(" -> ")
                .map(|pos| {
                    let (x, y) = pos.split_once(",").unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect_vec()
        })
        .collect_vec()
}

fn parse_rocks(raw_input: &Vec<Vec<(i32, i32)>>) -> RockMap {
    let mut rocks = HashMap::new();
    for rock_path in raw_input {
        let mut rock_cursor = rock_path[0];
        rocks.insert(rock_cursor, Material::Rock);
        for &path_corner in &rock_path[1..] {
            while rock_cursor != path_corner {
                if rock_cursor.0 == path_corner.0 {
                    rock_cursor.1 += signum(path_corner.1 - rock_cursor.1);
                } else {
                    rock_cursor.0 += signum(path_corner.0 - rock_cursor.0);
                }
                rocks.insert(rock_cursor, Material::Rock);
            }
        }
    }

    let rocks_x_pos = rocks.keys().map(|pos| pos.0).collect_vec();
    RockMap {
        x_range: (rocks_x_pos.iter().min().unwrap() - 1, rocks_x_pos.iter().max().unwrap() + 1),
        max_y: rocks.keys().map(|pos| pos.1).max().unwrap() + 1,
        material: rocks,
    }
}

#[allow(dead_code)]
pub fn day_14() {
    let input = read_input();

    {
        // Star 1
        let mut rock_map = parse_rocks(&input);
        while let Some(_) = rock_map.drop_sand_towards_void((X_SPAWN, Y_SPAWN)) {}
        //rock_map.pretty_print();
        println!("star 1: {}", rock_map.count_sand());
    }

    {
        // Star 2
        let mut rock_map = parse_rocks(&input);
        while !rock_map.material.contains_key(&(X_SPAWN, Y_SPAWN)) {
            rock_map.drop_sand_towards_floor((X_SPAWN, Y_SPAWN));
        }
        rock_map.pretty_print();
        println!("star 2: {}", rock_map.count_sand());
    }
}
