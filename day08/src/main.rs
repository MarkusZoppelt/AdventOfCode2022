use std::{cmp::max, io::Read};

//    0
// 2     3
//    1
fn solve(matrix: &Vec<Vec<usize>>, x: usize, y: usize) -> (bool, usize) {
    let height = matrix[x][y];
    let mut is_visible = vec![true; 4];

    // Check if it's an edge
    if x == 0 || y == matrix.len() - 1 || x == 0 || y == matrix[x].len() - 1 {
        return (true, 0);
    }

    let mut scores = vec![0; 4];

    for row in (0..x).rev() {
        scores[0] += 1;
        if matrix[row][y] >= height {
            is_visible[0] = false;
            break;
        }
    }

    for row in x + 1..matrix.len() {
        scores[1] += 1;
        if matrix[row][y] >= height {
            is_visible[1] = false;
            break;
        }
    }

    for col in matrix[x][0..y].iter().rev() {
        scores[2] += 1;
        if col >= &height {
            is_visible[2] = false;
            break;
        }
    }

    for col in &matrix[x][y + 1..] {
        scores[3] += 1;
        if col >= &height {
            is_visible[3] = false;
            break;
        }
    }

    let visible = is_visible[0] || is_visible[1] || is_visible[2] || is_visible[3];
    let scenic_score = scores.iter().product();
    (visible, scenic_score)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut visible_trees = 0;
    let mut max_score = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let (visible, score) = solve(&matrix, i, j);
            if visible {
                visible_trees += 1;
            }
            max_score = max(max_score, score);
        }
    }
    println!("Visible trees: {}", visible_trees);
    println!("Max scenic score: {}", max_score);
}
