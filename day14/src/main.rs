use std::collections::HashMap;
use std::io::Read;

fn solve(rocks: &HashMap<(i32, i32), ()>, limit_y: i32, bottom: bool) -> usize {
    let mut blocked = rocks.clone();
    let mut absyss = false;
    let mut stuck = false;
    while !absyss && !stuck {
        let (mut sx, mut sy) = (500, 0);
        while !absyss && !stuck {
            let mut moved = false;
            for &(dx, dy) in &[(0, 1), (-1, 1), (1, 1)] {
                let (nx, ny) = (sx + dx, sy + dy);
                if ny == limit_y && bottom {
                    break;
                }
                if !blocked.contains_key(&(nx, ny)) {
                    if ny >= limit_y {
                        absyss = true;
                        break;
                    } else {
                        sx = nx;
                        sy = ny;
                        moved = true;
                        break;
                    }
                }
            }
            if absyss {
                break;
            }
            if !moved {
                blocked.insert((sx, sy), ());
                if sx == 500 && sy == 0 {
                    stuck = true;
                }
                break;
            }
        }
    }
    blocked.len() - rocks.len()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut rocks = HashMap::new();
    let mut max_y = 0;

    for line in input.lines() {
        let points: Vec<Vec<i32>> = line
            .split(" -> ")
            .map(|p| p.split(",").map(|n| n.parse().unwrap()).collect())
            .collect();

        for i in 0..(points.len() - 1) {
            let (cx, cy) = (points[i][0], points[i][1]);
            let (nx, ny) = (points[i + 1][0], points[i + 1][1]);
            for x in (cx.min(nx))..(cx.max(nx) + 1) {
                for y in (cy.min(ny))..(cy.max(ny) + 1) {
                    rocks.insert((x, y), ());
                    if y > max_y {
                        max_y = y;
                    }
                }
            }
        }
    }

    let p1 = solve(&rocks, max_y, false);

    println!("Part I: {}", p1);

    let p2 = solve(&rocks, max_y + 2, true);
    println!("Part II: {}", p2);
}
