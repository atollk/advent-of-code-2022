use std::fs;

#[allow(dead_code)]
pub fn day_4() {
    let file_contents = fs::read_to_string("day4_puzzle.txt").expect("Unable to read file");
    let file_lines = file_contents.split("\n").collect::<Vec<&str>>();
    let elf_pairs = file_lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (sec1, sec2) = line.split_once(",").unwrap();
            let (sec1l, sec1r) = sec1.split_once("-").unwrap();
            let (sec2l, sec2r) = sec2.split_once("-").unwrap();
            (
                (sec1l.parse::<i32>().unwrap(), sec1r.parse::<i32>().unwrap()),
                (sec2l.parse::<i32>().unwrap(), sec2r.parse::<i32>().unwrap()),
            )
        })
        .collect::<Vec<_>>();
    let pair_overlaps = elf_pairs
        .iter()
        .map(|((sec1l, sec1r), (sec2l, sec2r))| {
            let overlaps_completely =
                (sec1l <= sec2l && sec1r >= sec2r) || (sec1l >= sec2l && sec1r <= sec2r);
            let overlaps_somewhere =
                (sec1l <= sec2l && sec2l <= sec1r) || (sec2l <= sec1l && sec1l <= sec2r);
            (overlaps_completely, overlaps_somewhere)
        })
        .collect::<Vec<_>>();

    println!("{:?}", pair_overlaps);
    println!("{:?}", pair_overlaps.iter().filter(|(all, _)| *all).count());
    println!("{:?}", pair_overlaps.iter().filter(|(_, any)| *any).count());
}
