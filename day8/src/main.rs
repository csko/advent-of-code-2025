use std::cmp::Reverse;
use std::u128;
use std::{fs, time::SystemTime};

const TOP_K: usize = 3;

#[derive(Debug, PartialEq)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}
impl Point {
    fn dist(&self, other: &Point) -> u128 {
        let Self {
            x: x1,
            y: y1,
            z: z1,
        } = self;
        let Self {
            x: x2,
            y: y2,
            z: z2,
        } = other;
        let dx = x1.abs_diff(*x2);
        let dy = y1.abs_diff(*y2);
        let dz = z1.abs_diff(*z2);

        (dx as u128) * (dx as u128) + (dy as u128) * (dy as u128) + (dz as u128) * (dz as u128)
    }
}

fn points_and_distances(data: &str) -> (Vec<Point>, Vec<(usize, usize, u128)>) {
    let mut points = vec![];

    for row in data.lines() {
        let nums: Vec<u64> = row.split(",").map(|f| f.parse().unwrap()).collect();
        let point = Point {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        };
        points.push(point);
    }
    let mut dist: Vec<Vec<u128>> = vec![vec![0; points.len()]; points.len()];
    for (i, point1) in points.iter().enumerate() {
        for (j, point2) in points.iter().enumerate() {
            let d = point1.dist(&point2);
            dist[i][j] = d;
            dist[j][i] = d;
        }
    }
    (points, build_sorted_dist_list(&dist))
}

fn build_sorted_dist_list(dist: &Vec<Vec<u128>>) -> Vec<(usize, usize, u128)> {
    let mut res = vec![];
    for (i, row) in dist.iter().enumerate() {
        for (j, cell) in row[i + 1..].iter().enumerate() {
            res.push((i, j + i + 1, *cell));
        }
    }
    // Distance decreasing order
    res.sort_by_key(|(_i, _j, dist)| Reverse(*dist));
    res
}

fn find_circuit(p: &Point, circuits: &Vec<Vec<&Point>>) -> Option<usize> {
    for (i, circuit) in circuits.iter().enumerate() {
        if circuit.contains(&p) {
            return Some(i);
        }
    }
    None
}

fn f1() -> anyhow::Result<()> {
    let data = fs::read_to_string("input1.txt")?;
    let (points, mut closest) = points_and_distances(&data);
    let max_rounds = 1000;
    let mut circuits = vec![];

    for _round in 0..max_rounds {
        let (i, j, _dist) = closest.pop().unwrap();

        // find both of their circuits
        let p1 = &points[i];
        let p2 = &points[j];
        let c1 = find_circuit(p1, &circuits);
        let c2 = find_circuit(p2, &circuits);
        match (c1, c2) {
            (None, None) => {
                circuits.push(vec![p1, p2]);
            }
            (None, Some(c2)) => {
                circuits[c2].push(p1);
            }
            (Some(c1), None) => {
                circuits[c1].push(p2);
            }
            (Some(c1), Some(c2)) => {
                if c1 != c2 {
                    let (c1, c2) = (std::cmp::min(c1, c2), std::cmp::max(c1, c2));
                    let mut removed = circuits.remove(c2);
                    circuits[c1].append(&mut removed);
                }
            }
        }
    }
    let mut lengths: Vec<_> = circuits.iter().map(|c| c.len()).collect();
    lengths.sort_by_key(|&x| Reverse(x));
    let sol = lengths[..TOP_K].iter().map(|x| *x).product::<usize>();
    println!("{sol}");
    Ok(())
}

fn f2() -> anyhow::Result<()> {
    let data = fs::read_to_string("input2.txt")?;
    let (points, mut closest) = points_and_distances(&data);
    let mut circuits = vec![];

    loop {
        let (i, j, _dist) = closest.pop().unwrap();

        let p1 = &points[i];
        let p2 = &points[j];
        let c1 = find_circuit(p1, &circuits);
        let c2 = find_circuit(p2, &circuits);
        match (c1, c2) {
            (None, None) => {
                circuits.push(vec![p1, p2]);
            }
            (None, Some(c2)) => {
                circuits[c2].push(p1);
            }
            (Some(c1), None) => {
                circuits[c1].push(p2);
            }
            (Some(c1), Some(c2)) => {
                if c1 != c2 {
                    let (c1, c2) = (std::cmp::min(c1, c2), std::cmp::max(c1, c2));

                    let mut removed = circuits.remove(c2);
                    circuits[c1].append(&mut removed);
                }
            }
        }
        if circuits[0].len() == points.len() {
            println!("{}", p1.x * p2.x);
            break;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let now = SystemTime::now();
    f1()?;
    println!("{:?}", now.elapsed()?);
    let now = SystemTime::now();
    f2()?;
    println!("{:?}", now.elapsed()?);
    Ok(())
}
