use std::io::prelude::*;

// create a struct elf that has a list of integers and function that return the sum of the integers
struct Elf {
    list: Vec<i32>,
}
impl Elf {
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in &self.list {
            sum += i;
        }
        sum
    }
}

fn main() {
    // loop over stdin and create elfs separated by a blank line
    // every line is a single integer that should be added to the elfs list
    let mut elfs = Vec::new();
    let mut elf = Elf { list: Vec::new() };
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "" {
            elfs.push(elf);
            elf = Elf { list: Vec::new() };
        } else {
            elf.list.push(line.parse().unwrap());
        }
    }
    // add the last elf 
    // (if there is no blank line at the end of the input)
    elfs.push(elf);

    // Part I
    // ======
    // loop over all elfs and print the sum of the elf that has the highest sum
    let mut max = 0;
    for elf in &elfs {
        let sum = elf.sum();
        if sum > max {
            max = sum;
        }
    }
    println!("{}", max);

    // Part II
    // =======
    // sort the list of elfs by the sum of the list ascending
    elfs.sort_by(|a, b| a.sum().cmp(&b.sum()));

    // get the sums of the last three elfs and add them
    let sum = elfs[elfs.len() - 1].sum() + elfs[elfs.len() - 2].sum() + elfs[elfs.len() - 3].sum();

    // print the sum
    println!("{}", sum);
}
