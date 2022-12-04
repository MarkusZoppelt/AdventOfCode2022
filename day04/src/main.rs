use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut count_part_i = 0;
    let mut count_part_ii = 0;

    for line in input.lines() {
        let mut pairs = line.split(",");
        let left = pairs.next().unwrap();
        let right = pairs.next().unwrap();

        let left_min = left.split("-").next().unwrap().parse::<u32>().unwrap();
        let left_max = left.split("-").nth(1).unwrap().parse::<u32>().unwrap();
        let right_min = right.split("-").next().unwrap().parse::<u32>().unwrap();
        let right_max = right.split("-").nth(1).unwrap().parse::<u32>().unwrap();

        if left_min <= right_min && right_max <= left_max
            || right_min <= left_min && left_max <= right_max
        {
            count_part_i += 1;
        }
        if left_min <= right_min && right_min <= left_max
            || right_min <= left_min && left_min <= right_max
        {
            count_part_ii += 1;
        }
    }
    println!("{}", count_part_i);
    println!("{}", count_part_ii);
}
