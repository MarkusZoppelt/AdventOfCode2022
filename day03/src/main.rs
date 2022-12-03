use std::io::prelude::*;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input_backup = input.clone();

    let mut prio_sum_i = 0;
    let mut prio_sum_ii = 0;

    // Part I
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let mut found = false;
        let mut common = String::new();
        for c in first.chars() {
            if second.contains(c) && !found {
                common.push(c);
                found = true;
            }
        }
        let c: char = common.chars().nth(0).unwrap();

        let prio;
        if c.is_lowercase() {
            prio = c as u64 - 'a' as u64 + 1;
        } else {
            prio = c as u64 - 'A' as u64 + 27;
        }
        prio_sum_i += prio;
    }
    println!("Solution Part I: {}", prio_sum_i);

    // Part II
    let mut lines = input_backup.lines();

    while let Some(first) = lines.next() {
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        let mut found = false;
        let mut common = String::new();
        for c in first.chars() {
            if second.contains(c) && third.contains(c) && !found {
                common.push(c);
                found = true;
            }
        }
        let c: char = common.chars().nth(0).unwrap();

        let prio;
        if c.is_lowercase() {
            prio = c as u64 - 'a' as u64 + 1;
        } else {
            prio = c as u64 - 'A' as u64 + 27;
        }
        prio_sum_ii += prio;
    }

    println!("Solution Part II: {}", prio_sum_ii);
}
