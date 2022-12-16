use itertools::Itertools;
use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{stdout, Write};
use std::ops::Index;
use num::pow;

const MINUTES: usize = 30;

#[derive(Debug, Clone)]
struct Valve {
    label: String,
    leads_to: Vec<String>,
    flow_rate: usize,
}

impl Valve {
    fn from_line(line: &str) -> Valve {
        let (pre_semi, post_semi) = line.split_once(";").unwrap();

        let mut iter1 = pre_semi.chars();
        iter1.by_ref().dropping(6);
        let valve_label = iter1.by_ref().take(2).join("");
        iter1.by_ref().dropping(15);
        let flow_rate = iter1.join("").parse().unwrap();

        let mut iter2 = post_semi.chars();
        iter2.by_ref().dropping(23);
        let tunnel_labels = iter2.join("").trim().split(", ").map(|s| s.to_string()).collect_vec();

        Valve { label: valve_label, flow_rate, leads_to: tunnel_labels }
    }
}

fn read_input() -> HashMap<String, Valve> {
    let file_contents = fs::read_to_string("day16_puzzle.txt").expect("Unable to read file");
    let mut valves = HashMap::new();
    for line in file_contents.split("\n") {
        let valve = Valve::from_line(line);
        valves.insert(valve.label.clone(), valve);
    }
    valves
}

#[allow(dead_code)]
pub fn day_16() {
    let valves = read_input();
    println!("{:?}", valves);
    let nonzero_valves = valves.iter().filter(|v| v.1.flow_rate > 0).map(|v| v.0.clone()).collect_vec();

    // Star 1
    {
        // (position, minutes, open valves) -> best score
        let mut best_release: HashMap<(&str, usize, u32), usize> = HashMap::new();

        for vs in 0u32..pow(2, nonzero_valves.len()) {
            for pos in valves.keys() {
                best_release.insert((pos, MINUTES, vs), 0);
            }
        }

        for minute in (0usize..MINUTES).rev() {
            print!("{} ", minute);
            stdout().flush();
            for pos in valves.keys() {
                for vs in 0u32..pow(2, nonzero_valves.len()) {
                    let score_move = valves[pos].leads_to.iter().map(|next_valve| {
                        let k = (next_valve.as_str(), minute + 1, vs);
                        best_release[&k]
                    }).max().unwrap();
                    let score_open =
                        {
                            let mut new_vs = vs;
                            let valve = &valves[pos];
                            if valve.flow_rate > 0 {
                                let valve_nz_pos = nonzero_valves.iter().find_position(|l| **l == valve.label).unwrap().0;
                                new_vs = new_vs | (1 << valve_nz_pos);
                            }
                            let k = (pos.as_str(), minute + 1, new_vs);
                            let next_score = best_release[&k];
                            if vs == new_vs {
                                next_score
                            } else {
                                next_score + (MINUTES - minute - 1) * valve.flow_rate
                            }
                        };
                    best_release.insert((pos, minute, vs), max(score_open, score_move));
                }
            }
        }

        println!("star 1: {:?}", best_release[&("AA", 0, 0)]);
    }

    // Star 2
    {
        // (position 1, postition 2, minutes, open valves) -> best score
        let mut best_release: HashMap<(&str, &str, usize, u32), usize> = HashMap::new();

        for vs in 0u32..pow(2, nonzero_valves.len()) {
            for pos1 in valves.keys() {
                for pos2 in valves.keys() {
                    best_release.insert((pos1, pos2, MINUTES, vs), 0);
                }
            }
        }

        for minute in (3usize..MINUTES).rev() {
            print!("{} ", minute);
            stdout().flush();
            for pos1 in valves.keys() {
                for pos2 in valves.keys() {
                    for vs in 0u32..pow(2, nonzero_valves.len()) {
                        let score_move_1 = valves[pos1].leads_to.iter().map(|next_valve1| {
                            let score_move = valves[pos2].leads_to.iter().map(|next_valve2| {
                                let k = (next_valve1.as_str(), next_valve2.as_str(), minute + 1, vs);
                                best_release[&k]
                            }).max().unwrap();
                            let score_open =
                                {
                                    let mut new_vs = vs;
                                    let valve = &valves[pos2];
                                    if valve.flow_rate > 0 {
                                        let valve_nz_pos = nonzero_valves.iter().find_position(|l| **l == valve.label).unwrap().0;
                                        new_vs = new_vs | (1 << valve_nz_pos);
                                    }
                                    let k = (pos1.as_str(), pos2.as_str(), minute + 1, new_vs);
                                    let next_score = best_release[&k];
                                    if vs == new_vs {
                                        next_score
                                    } else {
                                        next_score + (MINUTES - minute - 1) * valve.flow_rate
                                    }
                                };
                            max(score_open, score_move)
                        }).max().unwrap();
                        let score_move_2 = valves[pos2].leads_to.iter().map(|next_valve2| {
                            let mut new_vs = vs;
                            let valve = &valves[pos1];
                            if valve.flow_rate > 0 {
                                let valve_nz_pos = nonzero_valves.iter().find_position(|l| **l == valve.label).unwrap().0;
                                new_vs = new_vs | (1 << valve_nz_pos);
                            }
                            let k = (pos1.as_str(), pos2.as_str(), minute + 1, new_vs);
                            let next_score = best_release[&k];
                            if vs == new_vs {
                                next_score
                            } else {
                                next_score + (MINUTES - minute - 1) * valve.flow_rate
                            }
                        }).max().unwrap();
                        let score_open_both =
                            {
                                let mut new_vs = vs;
                                let valve1 = &valves[pos1];
                                let valve2 = &valves[pos1];
                                if valve1.flow_rate > 0 {
                                    let valve_nz_pos = nonzero_valves.iter().find_position(|l| **l == valve1.label).unwrap().0;
                                    new_vs = new_vs | (1 << valve_nz_pos);
                                }
                                if valve2.flow_rate > 0 && valve1.label != valve2.label {
                                    let valve_nz_pos = nonzero_valves.iter().find_position(|l| **l == valve2.label).unwrap().0;
                                    new_vs = new_vs | (1 << valve_nz_pos);
                                }
                                let k = (pos1.as_str(), pos2.as_str(), minute + 1, new_vs);
                                let mut next_score = best_release[&k];
                                if valve1.flow_rate > 0 {
                                    let valve_nz_pos = nonzero_valves.iter().find_position(|l| **l == valve1.label).unwrap().0;
                                    if (vs & (1 << valve_nz_pos)) == 0 {
                                        next_score += (MINUTES - minute - 1) * valve1.flow_rate;
                                    }
                                }
                                if valve2.flow_rate > 0 && valve1.label != valve2.label {
                                    let valve_nz_pos = nonzero_valves.iter().find_position(|l| **l == valve2.label).unwrap().0;
                                    if (vs & (1 << valve_nz_pos)) == 0 {
                                        next_score += (MINUTES - minute - 1) * valve2.flow_rate;
                                    }
                                }
                                next_score
                            };
                        best_release.insert((pos1, pos2, minute, vs), max(max(score_move_1, score_move_2), score_open_both));
                    }
                }
            }
        }

        println!("{:?}", best_release[&("AA", "AA", 25, 0)]);
        println!("{:?}", best_release[&("AA", "AA", 26, 0)]);
        println!("{:?}", best_release[&("AA", "AA", 27, 0)]);
        println!("{:?}", best_release[&("AA", "AA", 28, 0)]);
        println!("{:?}", best_release[&("AA", "AA", 29, 0)]);
        println!("{:?}", best_release[&("AA", "AA", 30, 0)]);
    }
}
