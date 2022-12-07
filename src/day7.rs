use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct FileTree {
    root_size: Option<usize>,
    children: HashMap<String, FileTree>,
}

#[derive(Debug)]
enum FileCommand {
    Cd(String),
    Ls(Vec<(Option<usize>, String)>),
}

impl FileCommand {
    fn parse_commands(lines: &[&str]) -> Vec<FileCommand> {
        let mut commands = Vec::new();
        let mut iter = lines.iter().peekable();
        loop {
            let next_command =
                if let Some(line) = iter.next() {
                    if line.starts_with("$ cd") {
                        let path = line.chars().dropping(5).as_str();
                        FileCommand::Cd(path.to_string())
                    } else {
                        let mut output_lines = Vec::new();
                        while let Some(i) = iter.next_if(|line| !line.starts_with("$")) {
                            output_lines.push(*i);
                        }
                        let output = output_lines
                            .into_iter()
                            .map(|line| {
                                let (size, name) = line.split_once(" ").unwrap();
                                (size.parse().ok(), name.to_string())
                            })
                            .collect_vec();
                        FileCommand::Ls(output)
                    }
                } else {
                    break;
                };
            commands.push(next_command);
        }
        commands
    }
}

impl FileTree {
    fn size(&self) -> usize {
        let children_sizes = self.children.values().map(|subtree| subtree.size()).sum::<usize>();
        children_sizes + self.root_size.unwrap_or(0)
    }

    fn recursive_walk(&self) -> FileTreeWalker {
        FileTreeWalker { stack: vec![&self] }
    }

    fn from_input(file_contents: &str) -> FileTree {
        let lines = file_contents.split("\n").filter(|line| !line.is_empty()).collect_vec();

        let commands = FileCommand::parse_commands(&lines);

        FileTree::from_commands(&commands)
    }

    fn from_commands(commands: &[FileCommand]) -> FileTree {
        let mut tree = FileTree { root_size: None, children: HashMap::new() };
        let mut current_path = Vec::new();

        for command in commands {
            match command {
                FileCommand::Cd(path) => {
                    match path.as_str() {
                        "/" => current_path = Vec::new(),
                        ".." => {
                            current_path.pop();
                            ()
                        }
                        _ => current_path.push(path)
                    }
                }
                FileCommand::Ls(children) => {
                    let mut subtree = &mut tree;
                    for &p in current_path.iter() {
                        subtree = subtree.children.get_mut(p).unwrap();
                    }
                    for child in children {
                        if !subtree.children.contains_key(&child.1) {
                            subtree.children.insert(child.1.clone(), FileTree { root_size: child.0, children: HashMap::new() });
                        }
                    }
                }
            }
        }

        tree
    }
}

struct FileTreeWalker<'a> {
    stack: Vec<&'a FileTree>,
}

impl<'a> Iterator for FileTreeWalker<'a> {
    type Item = &'a FileTree;

    fn next(&mut self) -> Option<Self::Item> {
        let next_subtree = self.stack.pop()?;
        self.stack.extend(next_subtree.children.values());
        Some(next_subtree)
    }
}

#[allow(dead_code)]
pub fn day_7() {
    let file_contents = fs::read_to_string("day7_puzzle.txt").expect("Unable to read file");

    let file_tree = FileTree::from_input(&file_contents);

    {
        // star 1
        let mut star1_sum = 0;
        for subtree in file_tree.recursive_walk() {
            let subtree_size = subtree.size();
            if subtree.root_size.is_none() && subtree_size <= 100000 {
                star1_sum += subtree_size;
            }
        }
        println!("{:?}", star1_sum);
    }

    // star 2
    {
        const TOTAL_SPACE: usize = 70000000;
        const TOTAL_REQUIRED_SPACE: usize = 30000000;
        let mut best_size = file_tree.size();
        let required_free_space = TOTAL_REQUIRED_SPACE - (TOTAL_SPACE - best_size);
        for subtree in file_tree.recursive_walk() {
            let subtree_size = subtree.size();
            if subtree.root_size.is_none() && subtree_size >= required_free_space && subtree_size < best_size {
                best_size = subtree_size;
            }
        }
        println!("free space needed: {:?}, solution: {:?}", required_free_space, best_size);
    }
}