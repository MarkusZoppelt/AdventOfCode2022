use std::io::Read;
use std::{cmp::Ordering, num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Num(i32),
}

fn parse(input: &str) -> (Vec<Packet>, Vec<Packet>) {
    let mut left = vec![];
    let mut right = vec![];

    for s in input.split("\n\n") {
        let (l, r) = s.trim().split_once('\n').unwrap();
        left.push(l.parse::<Packet>().unwrap());
        right.push(r.parse::<Packet>().unwrap());
    }

    (left, right)
}

fn parse_packet(s: &str) -> Result<(Packet, &str), ParseIntError> {
    let s = s.trim();
    if let Some(s) = s.strip_prefix("[") {
        let mut list = vec![];
        if s.starts_with("]") {
            return Ok((Packet::List(list), &s[1..]));
        }
        let mut rest = s;
        loop {
            let (value, remainder) = parse_packet(rest)?;
            list.push(value);
            rest = remainder;
            if rest.is_empty() {
                break;
            } else if rest.starts_with(',') {
                rest = &rest[1..];
            } else if rest.starts_with(']') {
                rest = &rest[1..];
                break;
            }
        }

        Ok((Packet::List(list), rest))
    } else {
        let (num, _) = s.split_once(|c| c == ' ' || c == ',' || c == ']').unwrap();
        Ok((Packet::Num(num.parse::<i32>()?), &s[num.len()..]))
    }
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, _) = parse_packet(s)?;
        Ok(value)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self, &other) {
            (Self::Num(i), Self::Num(j)) => i.cmp(j),
            (Self::Num(i), Self::List(_)) => {
                let list = Self::List(vec![Self::Num(*i)]);
                list.cmp(other)
            }
            (Self::List(_), Self::Num(j)) => {
                let list = Self::List(vec![Self::Num(*j)]);
                self.cmp(&list)
            }
            (Self::List(left), Self::List(right)) => {
                for (l, r) in left.iter().zip(right.iter()) {
                    match l.cmp(r) {
                        Ordering::Equal => {
                            continue;
                        }
                        other => {
                            return other;
                        }
                    }
                }
                left.len().cmp(&right.len())
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let (left, right) = parse(&input);

    let part_i: usize = left
        .iter()
        .zip(right.iter())
        .enumerate()
        .filter_map(|(i, (l, r))| match l.cmp(r) {
            Ordering::Less => Some(i + 1),
            _ => None,
        })
        .sum();

    println!("Part I: {}", part_i);

    let mut all = {
        let (mut left, mut right) = parse(&input);
        left.append(&mut right);
        left
    };
    all.sort();

    let start: Packet = "[[2]]".parse().unwrap();
    let end: Packet = "[[6]]".parse().unwrap();

    let (start, end) = (
        all.binary_search(&start).unwrap_err() + 1,
        all.binary_search(&end).unwrap_err() + 2,
    );

    let part_ii = start * end;
    println!("Part II: {}", part_ii);
}
