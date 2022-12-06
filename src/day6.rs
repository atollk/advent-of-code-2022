use std::fs;
use itertools::Itertools;
use iterslide::SlideIterator;

fn get_marker_position(file_contents: &str, window_size: usize) -> Option<usize> {
    for (i, window) in file_contents.chars().slide(window_size).enumerate() {
        if window.iter().unique().count() == window_size {
            return Some(i + window_size);
        }
    }
    None
}

#[allow(dead_code)]
pub fn day_6() {
    let file_contents = fs::read_to_string("day6_puzzle.txt").expect("Unable to read file");
    println!("{:?}", get_marker_position(&file_contents, 4).unwrap());
    println!("{:?}", get_marker_position(&file_contents, 14).unwrap());
}