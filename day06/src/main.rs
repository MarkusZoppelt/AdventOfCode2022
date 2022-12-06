use std::io::Read;

fn slide(s: &str, marker: usize) -> usize {
    let chars = s.char_indices().collect::<Vec<_>>();
    chars
        .windows(marker)
        .find(|&sub| {
            sub.iter().all(|(_, x)| {
                sub.len() - sub.iter().filter(|&(_, y)| y == x).count() == sub.len() - 1
            })
        })
        .map_or_else(|| 0, |w| w[w.len() - 1].0 + 1)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let lines = input.lines();

    for line in lines {
        let window_size_i = 4;
        let window_size_ii = 14;
        println!("{}", slide(line, window_size_i));
        println!("{}", slide(line, window_size_ii));
    }
}
