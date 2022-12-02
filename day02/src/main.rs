use ::std::io::BufRead;

fn play_game(move1: char, move2: char) -> i32 {
    let mut score = 0;
    if move1 == (move2 as u8 - 23) as char {
        println!("draw!");
        score += 3;
    } else if move1 == 'A' && move2 == 'Y' {
        score += 6;
    } else if move1 == 'B' && move2 == 'Z' {
        score += 6;
    } else if move1 == 'C' && move2 == 'X' {
        score += 6;
    } else {
        score += 0;
    }
    match move2 {
        'X' => score += 1,
        'Y' => score += 2,
        'Z' => score += 3,
        _ => (),
    }
    score
}

fn play_with_outcome(move1: char, outcome: char) -> i32 {
    let mut score = 0;
    let mut move2 = '?';

    if outcome == 'X' {
        // need to lose
        if move1 == 'A' {
            move2 = 'Z';
        } else if move1 == 'B' {
            move2 = 'X';
        } else if move1 == 'C' {
            move2 = 'Y';
        }
        score += play_game(move1, move2);
    } else if outcome == 'Y' {
        // need to draw
        if move1 == 'A' {
            move2 = 'X';
        } else if move1 == 'B' {
            move2 = 'Y';
        } else if move1 == 'C' {
            move2 = 'Z';
        }
        score += play_game(move1, move2);
    } else if outcome == 'Z' {
        // need to win
        if move1 == 'A' {
            move2 = 'Y';
        } else if move1 == 'B' {
            move2 = 'Z';
        } else if move1 == 'C' {
            move2 = 'X';
        }
        score += play_game(move1, move2);
    }
    score
}

fn main() {
    let lines = std::io::stdin().lock().lines().map(|l| l.unwrap());
    let mut score_part_i = 0;
    let mut score_part_ii = 0;

    for line in lines {
        let mut chars = line.split_whitespace();
        let opponent: char = chars.next().unwrap().chars().next().unwrap();
        let me: char = chars.next().unwrap().chars().next().unwrap();

        score_part_i += play_game(opponent, me);
        score_part_ii += play_with_outcome(opponent, me);
    }

    println!("{}", score_part_i);
    println!("{}", score_part_ii);
}
