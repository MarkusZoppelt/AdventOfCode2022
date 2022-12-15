use std::collections::HashSet;
use std::io::Read;

type Point = (i64, i64);
type Range = (i64, i64);
const WIDTH: i64 = 4000000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let y = 2000000;

    let mut sensors: HashSet<(Point, Point)> = HashSet::new();
    let mut beacons: HashSet<Point> = HashSet::new();
    let sensors_part_ii: Vec<(Point, Point, i64)> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(|c| c == '=' || c == ',' || c == ':').collect();
            let s_x: i64 = parts[1].parse().ok()?;
            let s_y: i64 = parts[3].parse().ok()?;
            let b_x: i64 = parts[5].parse().ok()?;
            let b_y: i64 = parts[7].parse().ok()?;
            sensors.insert(((s_x, s_y), (b_x, b_y)));
            beacons.insert((b_x, b_y));
            Some((
                (s_x, s_y),
                (b_x, b_y),
                (s_x - b_x).abs() + (s_y - b_y).abs(),
            ))
        })
        .collect();

    let mut ranges: HashSet<Range> = HashSet::new();
    let mut covered: HashSet<Point> = HashSet::new();

    for sensor in &sensors {
        let dist = (sensor.0 .0 - sensor.1 .0).abs() + (sensor.0 .1 - sensor.1 .1).abs();
        let y_diff = (sensor.0 .1 - y).abs();
        ranges.insert((sensor.0 .0 - (dist - y_diff), sensor.0 .0 + (dist - y_diff)));
    }

    for range in ranges {
        for n in range.0..range.1 + 1 {
            covered.insert((n, y));
        }
    }

    let c = beacons
        .iter()
        .fold(0, |acc, beacon| if beacon.1 == y { acc - 1 } else { acc })
        + covered
            .iter()
            .fold(0, |acc, cov| if cov.1 == y { acc + 1 } else { acc });

    println!("Part I: {}", c);

    let mut ranges: Vec<Range>;
    let mut x_coord = 0;
    let mut y_coord = 0;
    for y in 0..WIDTH + 1 {
        ranges = Vec::new();
        for sensor in &sensors_part_ii {
            let y_diff = (sensor.0 .1 - y).abs();
            let offset = sensor.2 - y_diff;
            let left = sensor.0 .0 - (offset).abs();
            let right = sensor.0 .0 + (offset).abs();
            if y_diff <= sensor.2 {
                if y_diff == sensor.2 {
                    ranges.push((sensor.0 .0, sensor.0 .0));
                } else {
                    ranges.push((left, right));
                }
            }
        }
        ranges.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let mut m = ranges[0].1;
        let mut uncovered: HashSet<Range> = HashSet::new();
        for range in ranges {
            uncovered.retain(|u| {
                if range.0 <= u.0 && range.1 >= u.1 {
                    return false;
                }
                return true;
            });
            if range.0 > m + 1 && range.0 < WIDTH - 1 {
                uncovered.insert((m + 1, range.0 - 1));
            }
            if range.1 >= m {
                m = range.1;
            }
        }
        if uncovered.len() > 0 {
            y_coord = y;
            for u in uncovered {
                x_coord = u.0;
            }
            break;
        }
    }

    println!("Part 2: {}", x_coord * WIDTH + y_coord);
}
