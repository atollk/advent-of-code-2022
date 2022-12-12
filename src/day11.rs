use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Clone)]
struct Day11State<'a> {
    monkey_programs: Vec<MonkeyProgram<'a>>,
}

#[derive(Clone)]
struct MonkeyProgram<'a> {
    items: VecDeque<u128>,
    operation: &'a dyn Fn(u128) -> u128,
    test_divisble_by: u32,
    true_test_target: usize,
    false_test_target: usize,
    inspect_count: usize,
}

impl<'a> Day11State<'a> {
    fn perform_round_div3(&mut self) {
        for i in 0..self.monkey_programs.len() {
            let item_count = self.monkey_programs[i].items.len();
            for _ in 0..item_count {
                self.monkey_programs[i].inspect_count += 1;
                let mut item = self.monkey_programs[i].items.pop_front().unwrap();
                item = (self.monkey_programs[i].operation)(item);
                item /= 3;
                let next_monkey = if (item % self.monkey_programs[i].test_divisble_by as u128) == 0
                {
                    self.monkey_programs[i].true_test_target
                } else {
                    self.monkey_programs[i].false_test_target
                };
                self.monkey_programs[next_monkey].items.push_back(item);
            }
        }
    }

    fn perform_round_unlimited(&mut self) {
        let div_test_product = self
            .monkey_programs
            .iter()
            .map(|mp| mp.test_divisble_by)
            .product::<u32>() as u128;
        for i in 0..self.monkey_programs.len() {
            let item_count = self.monkey_programs[i].items.len();
            for _ in 0..item_count {
                self.monkey_programs[i].inspect_count += 1;
                let mut item = self.monkey_programs[i].items.pop_front().unwrap();
                item = (self.monkey_programs[i].operation)(item);
                item %= div_test_product;
                let next_monkey = if (item % self.monkey_programs[i].test_divisble_by as u128) == 0
                {
                    self.monkey_programs[i].true_test_target
                } else {
                    self.monkey_programs[i].false_test_target
                };
                self.monkey_programs[next_monkey].items.push_back(item);
            }
        }
    }
}

impl<'a> Debug for MonkeyProgram<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MonkeyProgram {{ items: {:?}, inspect_count: {} }}",
            self.items, self.inspect_count
        )
    }
}

fn puzzle_input() -> Day11State<'static> {
    Day11State {
        monkey_programs: vec![
            MonkeyProgram {
                items: vec![66, 79].into_iter().collect(),
                operation: &(|old| old * 11),
                test_divisble_by: 7,
                true_test_target: 6,
                false_test_target: 7,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![84, 94, 94, 81, 98, 75].into_iter().collect(),
                operation: &(|old| old * 17),
                test_divisble_by: 13,
                true_test_target: 5,
                false_test_target: 2,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![85, 79, 59, 64, 79, 95, 67].into_iter().collect(),
                operation: &(|old| old + 8),
                test_divisble_by: 5,
                true_test_target: 4,
                false_test_target: 5,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![70].into_iter().collect(),
                operation: &(|old| old + 3),
                test_divisble_by: 19,
                true_test_target: 6,
                false_test_target: 0,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![57, 69, 78, 78].into_iter().collect(),
                operation: &(|old| old + 4),
                test_divisble_by: 2,
                true_test_target: 0,
                false_test_target: 3,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![65, 92, 60, 74, 72].into_iter().collect(),
                operation: &(|old| old + 7),
                test_divisble_by: 11,
                true_test_target: 3,
                false_test_target: 4,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![77, 91, 91].into_iter().collect(),
                operation: &(|old| old * old),
                test_divisble_by: 17,
                true_test_target: 1,
                false_test_target: 7,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![76, 58, 57, 55, 67, 77, 54, 99].into_iter().collect(),
                operation: &(|old| old + 6),
                test_divisble_by: 3,
                true_test_target: 2,
                false_test_target: 1,
                inspect_count: 0,
            },
        ],
    }
}

#[allow(dead_code)]
fn test_input() -> Day11State<'static> {
    Day11State {
        monkey_programs: vec![
            MonkeyProgram {
                items: vec![79, 98].into_iter().collect(),
                operation: &(|old| old * 19),
                test_divisble_by: 23,
                true_test_target: 2,
                false_test_target: 3,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![54, 65, 75, 74].into_iter().collect(),
                operation: &(|old| old + 6),
                test_divisble_by: 19,
                true_test_target: 2,
                false_test_target: 0,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![79, 60, 97].into_iter().collect(),
                operation: &(|old| old * old),
                test_divisble_by: 13,
                true_test_target: 1,
                false_test_target: 3,
                inspect_count: 0,
            },
            MonkeyProgram {
                items: vec![74].into_iter().collect(),
                operation: &(|old| old + 3),
                test_divisble_by: 17,
                true_test_target: 0,
                false_test_target: 1,
                inspect_count: 0,
            },
        ],
    }
}

#[allow(dead_code)]
pub fn day_11() {
    let initial_state = puzzle_input();
    println!("{:?}", initial_state);

    // Star 1
    {
        let mut state = initial_state.clone();
        for _ in 0..20 {
            state.perform_round_div3();
        }
        let monkey_inspects = state
            .monkey_programs
            .iter()
            .map(|mp| mp.inspect_count)
            .sorted_by(|a, b| b.cmp(a))
            .collect_vec();
        println!("Star 1: {}", monkey_inspects[0] * monkey_inspects[1]);
    }

    // Star 2
    {
        let mut state = initial_state.clone();
        for _ in 0..10_000 {
            state.perform_round_unlimited();
        }
        let monkey_inspects = state
            .monkey_programs
            .iter()
            .map(|mp| mp.inspect_count)
            .sorted_by(|a, b| b.cmp(a))
            .collect_vec();
        println!("Star 2: {}", monkey_inspects[0] * monkey_inspects[1]);
    }
}
