use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn adjacents(&self) -> [Self; 6] {
        [
            Self::new(self.x - 1, self.y, self.z),
            Self::new(self.x + 1, self.y, self.z),
            Self::new(self.x, self.y + 1, self.z),
            Self::new(self.x, self.y - 1, self.z),
            Self::new(self.x, self.y, self.z + 1),
            Self::new(self.x, self.y, self.z - 1),
        ]
    }

    fn from_str(line: &str) -> Self {
        let mut coords = line.trim().split(',').map(|n| n.parse().unwrap());
        Self::new(
            coords.next().unwrap(),
            coords.next().unwrap(),
            coords.next().unwrap(),
        )
    }
}

fn expand(s: Cube, map: &HashSet<Cube>, visited: &mut HashSet<Cube>, end: Cube) -> usize {
    let mut queue = VecDeque::new();
    let mut sum = 0;

    queue.push_back(s);

    while let Some(start) = queue.pop_front() {
        if visited.contains(&start)
            || start.x > end.x
            || start.x < s.x
            || start.y > end.y
            || start.y < s.y
            || start.z > end.z
            || start.z < s.z
        {
            continue;
        }

        visited.insert(start);
        for n in start.adjacents().into_iter() {
            if map.contains(&n) {
                sum += 1;
            } else {
                queue.push_back(n);
            }
        }
    }
    sum
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let cubes: HashSet<Cube> = input.lines().map(Cube::from_str).collect();
    let p1: usize = cubes
        .iter()
        .map(|cube| {
            cube.adjacents()
                .iter()
                .filter(|adj| !cubes.contains(adj))
                .count()
        })
        .sum();
    println!("Part I: {}", p1);

    let cubes: HashSet<Cube> = input.lines().map(Cube::from_str).collect();
    let (mut lx, mut ly, mut lz) = (0, 0, 0);
    cubes
        .iter()
        .for_each(|cube| (lx, ly, lz) = (lx.max(cube.x), ly.max(cube.y), lz.max(cube.z)));
    let end = Cube::new(lx + 1, ly + 1, lz + 1);
    let mut visited = HashSet::new();
    let p2: usize = expand(Cube::new(-1, -1, -1), &cubes, &mut visited, end);
    println!("Part II: {}", p2);
}
