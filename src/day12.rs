use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct InputMap {
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    height_map: Vec<Vec<u8>>,
}

fn read_input() -> InputMap {
    let file_contents = fs::read_to_string("day12_puzzle.txt").expect("Unable to read file");
    let mut result = InputMap { start_pos: (0, 0), end_pos: (0, 0), height_map: Vec::new() };
    for (i, line) in file_contents.split("\n").enumerate() {
        result.height_map.push(Vec::new());
        for (j, c) in line.char_indices() {
            let height = if c == 'S' {
                result.start_pos = (i, j);
                0
            } else if c == 'E' {
                result.end_pos = (i, j);
                26
            } else {
                (c as u32 - 'a' as u32 + 1) as u8
            };
            result.height_map[i].push(height);
        }
    }
    result
}

#[allow(dead_code)]
pub fn day_12() {
    let input_map = read_input();
    let col_n = input_map.height_map.len();
    let row_n = input_map.height_map[0].len();

    // run Dijkstra's algorithm starting from E
    let distances = {
        let mut distances = vec![vec![u32::MAX - 1; row_n as usize]; col_n as usize];
        distances[input_map.end_pos.0][input_map.end_pos.1] = 0;

        let mut stack = vec![input_map.end_pos];
        while !stack.is_empty() {
            let to_pos = stack.pop().unwrap();
            let from_poss = vec![
                (to_pos.0 as i32 + 1, to_pos.1 as i32),
                (to_pos.0 as i32, to_pos.1 as i32 + 1),
                (to_pos.0 as i32 - 1, to_pos.1 as i32),
                (to_pos.0 as i32, to_pos.1 as i32 - 1),
            ]
                .into_iter()
                .filter(|&(c, r)| c >= 0 && r >= 0 && c < col_n as i32 && r < row_n as i32)
                .map(|(c, r)| (c as usize, r as usize));
            for from_pos in from_poss {
                if input_map.height_map[to_pos.0][to_pos.1] as i32 - input_map.height_map[from_pos.0][from_pos.1] as i32 > 1 {
                    continue;
                }
                let from_pos_dist = distances[to_pos.0][to_pos.1] + 1;
                let to_pos_dist = &mut distances[from_pos.0][from_pos.1];
                if *to_pos_dist > from_pos_dist {
                    *to_pos_dist = from_pos_dist;
                    stack.push(from_pos);
                }
            }
        }
        distances
    };
    println!("{:?}", distances);

    println!("star 1 : {}", distances[input_map.start_pos.0][input_map.start_pos.1]);

    println!("star 2: {}",
             (0..col_n).cartesian_product(0..row_n)
                 .filter(|&(c, r)| input_map.height_map[c][r] == 1)
                 .map(|(c, r)| distances[c][r])
                 .min()
                 .unwrap()
    );
}