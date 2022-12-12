use std::{
    collections::{HashMap, VecDeque},
    io::prelude::*,
};

fn get_dist(grid: &HashMap<(i16, i16), char>, start: (i16, i16), end: (i16, i16)) -> Option<u16> {
    let mut visited: HashMap<(i16, i16), u16> = HashMap::new();
    let mut to_visit: VecDeque<(i16, i16)> = VecDeque::new();

    visited.insert(start, 0);
    to_visit.push_back(start);

    while !to_visit.is_empty() {
        let (cx, cy) = to_visit.pop_front().unwrap();

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = ((cx + dx), (cy + dy));
            if grid.contains_key(&(nx, ny)) && !visited.contains_key(&(nx, ny)) {
                let current_char = grid.get(&(cx, cy)).unwrap();
                let next_char = grid.get(&(nx, ny)).unwrap();
                if *next_char as i16 - *current_char as i16 <= 1 {
                    to_visit.push_back((nx, ny));
                    visited.insert((nx, ny), visited.get(&(cx, cy)).unwrap() + 1);

                    if (nx, ny) == end {
                        return Some(visited.get(&(cx, cy)).unwrap() + 1);
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut grid: HashMap<(i16, i16), char> = HashMap::new();
    let mut start = (255, 255);
    let mut end = (255, 255);

    let mut possible_start: Vec<(i16, i16)> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut height = c;
            if c.eq(&'S') {
                start = (x as i16, y as i16);
                height = 'a';
            } else if c.eq(&'E') {
                end = (x as i16, y as i16);
                height = 'z';
            }
            if height.eq(&'a') {
                possible_start.push((x as i16, y as i16));
            }
            grid.insert((x as i16, y as i16), height);
        }
    }

    // Part I
    println!("{}", get_dist(&grid, start, end).unwrap());

    // Part II
    let mut min = u16::MAX;
    possible_start.into_iter().for_each(|start| {
        if let Some(dist) = get_dist(&grid, start, end) {
            if dist < min {
                min = dist;
            }
        }
    });

    println!("{}", min);
}
