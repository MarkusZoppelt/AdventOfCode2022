use std::io::{self, Read};

fn decrypt(mut numbers: Vec<(i64, usize)>, cycles: i64) -> i64 {
    let size = numbers.len() as i64 - 1;

    for _ in 0..cycles {
        for current in 0..numbers.len() {
            let i = numbers.iter().position(|x| x.1 == current).unwrap();
            let mut new_i = i as i64 + numbers[i].0;
            new_i = ((new_i % size) + size) % size;

            let number = numbers.remove(i);
            numbers.insert(new_i as usize, number);
        }
    }

    let zero_i = numbers.iter().position(|x| x.0 == 0).unwrap();
    let n1 = numbers[(zero_i + 1000) % numbers.len()].0;
    let n2 = numbers[(zero_i + 2000) % numbers.len()].0;
    let n3 = numbers[(zero_i + 3000) % numbers.len()].0;
    n1 + n2 + n3
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let numbers: Vec<(i64, usize)> = input
        .lines()
        .enumerate()
        .map(|(orig_pos, line)| (line.parse::<i64>().unwrap() * 1, orig_pos))
        .collect();
    let result = decrypt(numbers, 1);
    println!("Part I: {}", result);

    let numbers: Vec<(i64, usize)> = input
        .lines()
        .enumerate()
        .map(|(orig_pos, line)| (line.parse::<i64>().unwrap() * 811_589_153, orig_pos))
        .collect();
    let result = decrypt(numbers, 10);
    println!("Part II: {}", result);
}
