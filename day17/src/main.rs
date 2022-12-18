use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;

enum Draft {
    Left,
    Right,
}

const P1_ITERATIONS: usize = 2022;
const P2_ITERATIONS: usize = 1_000_000_000_000;

const WIDTH: usize = 7;
const MAX_HEIGHT: usize = 8000;

type Rock = [u8; 16];
type Grid = [u8; WIDTH * MAX_HEIGHT];

fn fits(rock: &Rock, grid: &Grid, x0: usize, y0: usize) -> bool {
    for i in 0..16 {
        let x = i % 4;
        let y = i / 4;
        if rock[i] == 1 {
            if grid[(y0 + y) * WIDTH + (x0 + x)] == 1 {
                return false;
            }
        }
    }
    true
}

#[derive(Hash, PartialEq, Eq)]
struct Configuration {
    rock: usize,
    draft: usize,
    grid: u128,
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let drafts: Vec<Draft> = input
        .chars()
        .filter_map(|c| match c {
            '>' => Some(Draft::Right),
            '<' => Some(Draft::Left),
            _ => None,
        })
        .collect();

    let rocks = [
        ((4, 1), [1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        ((3, 3), [0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0]),
        ((3, 3), [1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0]),
        ((1, 4), [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]),
        ((2, 2), [1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    ];

    let mut grid: Grid = [0u8; WIDTH * MAX_HEIGHT];
    let mut draft_source = drafts.iter().enumerate().cycle();
    let mut floor = 0;
    let mut seen: HashMap<Configuration, (usize, usize)> = HashMap::new();
    let mut p1: Option<usize> = None;
    let mut p2: Option<usize> = None;
    for (i, (rock_index, ((width, height), rock))) in
        (0..P2_ITERATIONS).zip(rocks.iter().enumerate().cycle())
    {
        let mut y = floor + 3;
        let mut x: i32 = 2;
        let draft_index = loop {
            let (draft_index, draft) = draft_source.next().unwrap();

            match draft {
                Draft::Left => {
                    if x > 0 && fits(rock, &grid, (x - 1) as usize, y as usize) {
                        x -= 1;
                    }
                }
                Draft::Right => {
                    if x + width < WIDTH as i32 && fits(rock, &grid, (x + 1) as usize, y as usize) {
                        x += 1
                    }
                }
            }
            if y == 0 {
                break draft_index;
            }
            if fits(rock, &grid, x as usize, (y - 1) as usize) {
                y -= 1;
            } else {
                break draft_index;
            }
        };
        {
            let grid: &mut Grid = &mut grid;
            let x0 = x as usize;
            let y0 = y as usize;
            for i in 0..16 {
                let x = i % 4;
                let y = i / 4;
                if rock[i] == 1 {
                    grid[(y0 + y) * WIDTH + (x0 + x)] = 1;
                }
            }
        };

        floor = floor.max(y + height);

        if floor > 18 {
            let start = floor - 17;
            let configuration = Configuration {
                rock: rock_index,
                draft: draft_index,
                grid: {
                    let grid_part: &[u8] = &grid[start * WIDTH..(start + 18) * WIDTH];
                    let mut v = 0u128;
                    for i in 0..(18 * WIDTH) {
                        if grid_part[i] == 1 {
                            v += 1u128 << i;
                        }
                    }
                    v
                },
            };
            if let Some((iteration, floor_height)) = seen.get(&configuration) {
                let iteration_period = i - iteration;
                if P2_ITERATIONS % iteration_period == i % iteration_period {
                    let floor_period = floor - floor_height;
                    let remaining = (P2_ITERATIONS - i) / iteration_period;
                    p2 = Some(floor + remaining * floor_period - 1);
                }
            } else {
                seen.insert(configuration, (i, floor));
            }
        }

        if i == P1_ITERATIONS - 1 {
            p1 = Some(floor);
        }
        if i > P1_ITERATIONS && p2.is_some() {
            break;
        }
    }

    println!("Part I: {}", p1.unwrap());
    println!("Part II: {}", p2.unwrap());
}
