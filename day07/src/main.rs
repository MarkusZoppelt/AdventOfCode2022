use std::{
    collections::{HashMap, HashSet},
    io::Read,
    path::PathBuf,
};

fn calc_dir_size(
    tree: &HashMap<PathBuf, HashSet<(i64, &str)>>,
    sizes: &mut HashMap<PathBuf, i64>,
    dir: &PathBuf,
) {
    if sizes.contains_key(dir) {
        return;
    }
    let size = tree[dir]
        .iter()
        .map(|&(s, d)| match s {
            -1 => {
                let dir = dir.join(d);
                calc_dir_size(tree, sizes, &dir);
                sizes[&dir]
            }
            s => s,
        })
        .sum();
    sizes.insert(dir.clone(), size);
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut tree = HashMap::new();
    let mut sizes = HashMap::new();
    let mut current_dir = PathBuf::new();

    for lines in input.split('$').skip(1) {
        match lines.trim().lines().next().unwrap() {
            "ls" => {
                let entries = lines.lines().skip(1).map(|output| {
                    let (size, filename) = output.split_once(' ').unwrap();
                    (size.parse::<i64>().unwrap_or(-1), filename)
                });
                tree.entry(current_dir.clone())
                    .or_insert(HashSet::new())
                    .extend(entries);
            }
            "cd .." => {
                current_dir.pop();
            }
            cd_dir => {
                current_dir.push(cd_dir.split_once(' ').unwrap().1);
            }
        }
    }

    for node in tree.keys() {
        calc_dir_size(&tree, &mut sizes, node);
    }
    let part_i = sizes.values().filter(|&&s| s <= 100000).sum::<i64>();
    println!("{}", part_i);

    let total = sizes[&PathBuf::from("/")];
    let part_ii = sizes
        .values()
        .filter(|&&s| 40000000 + s >= total)
        .min()
        .copied()
        .unwrap();
    println!("{}", part_ii);
}
