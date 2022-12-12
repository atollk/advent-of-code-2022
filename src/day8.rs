use std::cmp::max;
use std::fs;

fn read_input() -> Vec<Vec<u8>> {
    let file_contents = fs::read_to_string("day8_puzzle.txt").expect("Unable to read file");
    let mut result = vec![Vec::new()];
    for c in file_contents.chars() {
        if c == '\n' {
            result.push(Vec::new());
        } else {
            let last_row = result.last_mut().unwrap();
            last_row.push((c as u8) - b'0');
        }
    }
    result
}

enum Rotation {
    D0,
    D90,
    D180,
    D270,
}

fn rotate_tree_grid<T: Default + Clone>(
    tree_grid: &Vec<Vec<T>>,
    rotation: Rotation,
) -> Vec<Vec<T>> {
    let n = tree_grid.len();
    let mut result = vec![vec![T::default(); n]; n];
    for r in 0..n {
        for c in 0..n {
            let (new_r, new_c) = match rotation {
                Rotation::D0 => (r, c),
                Rotation::D90 => (c, n - 1 - r),
                Rotation::D180 => (n - 1 - r, n - 1 - c),
                Rotation::D270 => (n - 1 - c, r),
            };
            result[new_r][new_c] = tree_grid[r][c].clone();
        }
    }
    result
}

fn perform_grid_op_4directional<T: Default + Clone, R: Default + Clone>(
    tree_grid: &Vec<Vec<T>>,
    f: &dyn Fn(&Vec<Vec<T>>) -> Vec<Vec<R>>,
) -> (Vec<Vec<R>>, Vec<Vec<R>>, Vec<Vec<R>>, Vec<Vec<R>>) {
    (
        rotate_tree_grid(&f(&rotate_tree_grid(tree_grid, Rotation::D0)), Rotation::D0),
        rotate_tree_grid(
            &f(&rotate_tree_grid(tree_grid, Rotation::D90)),
            Rotation::D270,
        ),
        rotate_tree_grid(
            &f(&rotate_tree_grid(tree_grid, Rotation::D180)),
            Rotation::D180,
        ),
        rotate_tree_grid(
            &f(&rotate_tree_grid(tree_grid, Rotation::D270)),
            Rotation::D90,
        ),
    )
}

fn count_visible_trees(tree_grid: &Vec<Vec<u8>>) -> usize {
    let n = tree_grid.len();

    let is_visible_from_left_to_right_map = |tree_grid: &Vec<Vec<u8>>| {
        let mut result = vec![vec![false; n]; n];
        for r in 0..n {
            let mut max_height = -1;
            for c in 0..n {
                let height = tree_grid[r][c] as i32;
                if height > max_height {
                    max_height = height;
                    result[r][c] = true;
                }
            }
        }
        result
    };

    let (visibility_map_left, visibility_map_top, visibility_map_right, visibility_map_down) =
        perform_grid_op_4directional(&tree_grid, &is_visible_from_left_to_right_map);

    let mut visible_trees = 0;
    for r in 0..n {
        for c in 0..n {
            let visible = visibility_map_left[r][c]
                || visibility_map_right[r][c]
                || visibility_map_top[r][c]
                || visibility_map_down[r][c];
            if visible {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn compute_scenic_score(tree_grid: &Vec<Vec<u8>>) -> Vec<Vec<u32>> {
    let n = tree_grid.len();

    let view_range_from_left_to_right_map = |tree_grid: &Vec<Vec<u8>>| {
        let mut result = vec![vec![0; n]; n];
        for h in 0..=9 {
            for r in 0..n {
                let mut tiles_since = -1;
                for c in 0..n {
                    tiles_since += 1;
                    let height = tree_grid[r][c] as i32;
                    if height >= h {
                        if height == h {
                            result[r][c] = tiles_since as u32;
                        }
                        tiles_since = 0;
                    }
                }
                result[r][n - 1] = max(result[r][n - 1], tiles_since as u32);
            }
        }
        result
    };

    let (view_range_left, view_range_top, view_range_right, view_range_down) =
        perform_grid_op_4directional(tree_grid, &view_range_from_left_to_right_map);

    let mut scenic_score = vec![vec![0; n]; n];
    for r in 0..n {
        for c in 0..n {
            let score = view_range_left[r][c]
                * view_range_top[r][c]
                * view_range_right[r][c]
                * view_range_down[r][c];
            scenic_score[r][c] = score;
        }
    }

    scenic_score
}

#[allow(dead_code)]
pub fn day_8() {
    let tree_grid = read_input();
    assert_eq!(tree_grid.len(), tree_grid.first().unwrap().len());
    println!("{:?}", tree_grid);

    // star 1
    println!("{:?}", count_visible_trees(&tree_grid));

    // star 2
    println!(
        "{:?}",
        compute_scenic_score(&tree_grid).iter().flatten().max()
    );
}
