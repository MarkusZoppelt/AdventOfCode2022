use itertools::Itertools;
use std::io::Read;

fn tick(cycle_count: &mut i32, signal_strength: &mut i32, screen: &mut Vec<char>, x: i32) {
    let pos = *cycle_count % screen.len() as i32;

    let pixel_on = x >= pos % 40 - 1 && x <= pos % 40 + 1;
    if pixel_on {
        screen[pos as usize] = '#';
    } else {
        screen[pos as usize] = ' ';
    }

    *cycle_count += 1;

    if *cycle_count >= 20 && (*cycle_count - 20) % 40 == 0 {
        *signal_strength += *cycle_count * x;
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines();

    let mut x = 1;
    let mut cycle_count = 0;
    let mut signal_strength = 0;

    let mut screen: Vec<char> = Vec::new();

    // fill every pixel with a space
    for _ in 0..40 * 6 {
        screen.push(' ');
    }

    for line in lines {
        if line.eq("noop") {
            tick(&mut cycle_count, &mut signal_strength, &mut screen, x);
        } else {
            let (_, value) = line.split_once(' ').unwrap();
            let value = value.parse::<i32>().unwrap();

            tick(&mut cycle_count, &mut signal_strength, &mut screen, x);
            tick(&mut cycle_count, &mut signal_strength, &mut screen, x);
            x += value;
        }
    }

    // Part 1
    println!("Finished after {} cycles. X: {}", cycle_count, x);
    println!("Signal strength: {}", signal_strength);

    // Part 2
    let screen = screen
        .chunks(40 as usize)
        .map(|x| x.iter().join(""))
        .join("\n");
    println!("{}", screen);
}
