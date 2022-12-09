use std::{collections::HashSet, io::Read};

fn follow(head_pos: (i32, i32), tail: &mut (i32, i32)) {
    let (x_dist, y_dist) = (head_pos.0 - tail.0, head_pos.1 - tail.1);
    if x_dist.abs() > 1 || y_dist.abs() > 1 {
        tail.0 += x_dist.signum();
        tail.1 += y_dist.signum();
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.lines();
    let mut head = (0, 0) as (i32, i32);
    let mut tails = [(0, 0); 9];

    let mut tail_visited_i: HashSet<(i32, i32)> = HashSet::new();
    let mut tail_visited_ii: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let (motion, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<u32>().unwrap();

        for _ in 0..steps {
            match motion {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                _ => panic!("unknown motion"),
            }

            follow(head, &mut tails[0]);

            tail_visited_i.insert(tails[0]);

            for i in 1..9 {
                let (left, right) = tails.split_at_mut(i);
                follow(left[i - 1], &mut right[0]);
            }
            tail_visited_ii.insert(tails[8]);
        }
    }

    println!("Part I: {}", tail_visited_i.len());
    println!("Part II: {}", tail_visited_ii.len());
}
