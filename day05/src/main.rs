#![allow(unused_assignments)]
use std::{collections::VecDeque, io::Read};

#[derive(PartialEq)]
enum Mode {
    Building,
    Moving,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines();

    let mut result = String::new();

    let mut first = true;
    let mut line_len = 0;
    let mut stacks: Vec<VecDeque<_>> = Vec::new();

    let mut mode = Mode::Building;
    const CRATEMOVER_9000: bool = true;

    for line in lines {
        match mode {
            Mode::Building => {
                if first {
                    line_len = line.len();
                    let num_stacks = (line_len + 1) / 4;
                    stacks = vec![VecDeque::<char>::new(); num_stacks];
                    mode = Mode::Building;
                    first = false;
                }
                if line.is_empty() {
                    mode = Mode::Moving;
                    continue;
                }
                if line.starts_with(" 1 ") {
                    continue;
                }
                for (i, c) in line.chars().enumerate() {
                    if c != '[' && c != ']' && c != ' ' {
                        stacks[i / 4].push_front(c);
                    }
                }
            }
            Mode::Moving => {
                let mut parts = line.split_whitespace();
                let mut num_to_move = parts.nth(1).unwrap().parse::<usize>().unwrap();
                let from = parts.nth(1).unwrap().parse::<usize>().unwrap();
                let to = parts.nth(1).unwrap().parse::<usize>().unwrap();

                if CRATEMOVER_9000 {
                    while num_to_move > 0 {
                        let c = stacks[from - 1].pop_back().unwrap();
                        stacks[to - 1].push_back(c);
                        num_to_move -= 1;
                    }
                } else {
                    let mut to_move = Vec::new();
                    for _ in 0..num_to_move {
                        to_move.push(stacks[from - 1].pop_back().unwrap());
                    }
                    for c in to_move.into_iter().rev() {
                        stacks[to - 1].push_back(c);
                    }
                }
            }
        }
    }
    for stack in &stacks {
        result.push(stack[stack.len() - 1]);
    }
    println!("{}", result);
}
