use regex::Regex;
use std::collections::VecDeque;
use std::{env, fs, process};

// const WORRY_REDUCE: u64 = 1;
const ROUNDS: u64 = 10000;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: VecDeque<u64>,
    pub op: Operation,
    pub divisible: u64,
    pub true_dest: u64,
    pub false_dest: u64,
    pub inspections: u64,
}

fn play_round(monkeys: &mut [Monkey], modulo: u64) {
    for i in 0..monkeys.len() {
        unsafe {
            let monkey: *mut Monkey = &mut monkeys[i];
            while (*monkey).items.len() > 0 {
                let mut item: u64 = (*monkey).items.pop_front().unwrap();
                (*monkey).inspections += 1;
                match (*monkey).op {
                    Operation::Add(n) => {
                        item += n;
                    }
                    Operation::Multiply(n) => {
                        item *= n;
                    }
                    Operation::Square => {
                        item *= item;
                    }
                }
                item %= modulo;
                if item % ((*monkey).divisible) == 0 {
                    let other_monkey: *mut Monkey = &mut monkeys[(*monkey).true_dest as usize];
                    (*other_monkey).items.push_back(item.clone());
                } else {
                    let other_monkey: *mut Monkey = &mut monkeys[(*monkey).false_dest as usize];
                    (*other_monkey).items.push_back(item.clone());
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let mut monkeys: Vec<Monkey> = vec![];
    let start_regex = Regex::new(r"^  Starting items: (\d+(, \d+)*)").unwrap();
    let op_regex =
        Regex::new(r"^  Operation: new = (old \* old|old \* (\d+)|old \+ (\d+))").unwrap();
    let test_regex = Regex::new(r"^  Test: divisible by (\d+)").unwrap();
    let true_regex = Regex::new(r"^    If true: throw to monkey (\d+)").unwrap();
    let false_regex = Regex::new(r"^    If false: throw to monkey (\d+)").unwrap();
    let filename: &str = &args[1];
    let contents = fs::read_to_string(&filename)
        .unwrap_or_else(|error| panic!("error reading {}: {:?}", &filename, &error));
    let mut lines = contents.lines();
    loop {
        let line = lines.next().expect("main(): unexpected end of input!");
        if !line.starts_with("Monkey") {
            panic!("main(): expected line starting with: `Monkey'");
        }
        let mut line = lines.next().expect("main(): unexpected end of input!");
        let mut caps = start_regex
            .captures(&line)
            .expect("main(): expected line starting with `  Starting items:'");
        let items: VecDeque<u64> = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
        line = lines.next().expect("main(): unexpected end of input!");
        caps = op_regex
            .captures(&line)
            .expect("main(): expected line starting with `  Operation:'");
        let op_str = caps.get(1).unwrap().as_str();
        let op: Operation;
        if op_str == "old * old" {
            op = Operation::Square;
        } else if op_str.starts_with("old *") {
            let n = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            op = Operation::Multiply(n);
        } else {
            // op_str.starts_with("old +")
            let n = caps.get(3).unwrap().as_str().parse::<u64>().unwrap();
            op = Operation::Add(n);
        }
        line = lines.next().expect("main(): unexpected end of input!");
        caps = test_regex
            .captures(&line)
            .expect("main(): expected line starting with `  Test:'");
        let divisible: u64 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        line = lines.next().expect("main(): unexpected end of input!");
        caps = true_regex
            .captures(&line)
            .expect("main(): expected line starting with `    If true:'");
        let true_dest: u64 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        line = lines.next().expect("main(): unexpected end of input!");
        caps = false_regex
            .captures(&line)
            .expect("main(): expected line starting with `    If false:'");
        let false_dest: u64 = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
        monkeys.push(Monkey {
            items,
            op,
            divisible,
            true_dest,
            false_dest,
            inspections: 0,
        });
        let maybe_line = lines.next();
        match maybe_line {
            None => {
                break;
            }
            Some(l) => {
                if l != "" {
                    panic!("main(): expected blank line");
                }
            }
        }
    }
    let modulo: u64 = monkeys.iter().map(|x| x.divisible).product();
    for _ in 1..=ROUNDS {
        play_round(&mut monkeys.as_mut_slice(), modulo);
    }
    let mut inspections: Vec<u64> = monkeys.iter().map(|x| x.inspections).collect();
    inspections.sort_by(|a, b| b.cmp(a));
    let first: u64 = inspections[0];
    let second: u64 = inspections[1];
    let monkey_business: u64 = first * second;
    println!("monkey business = {}", monkey_business);
}
