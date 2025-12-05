use std::{fs, time::SystemTime};

fn contains(ranges: &[(u64, u64)], n: u64) -> bool {
    ranges
        .binary_search_by_key(&n, |(left, _)| *left)
        .map_or_else(
            |i| {
                i > 0
                    && ranges[..i]
                        .iter()
                        .rev()
                        .take_while(|(left, _)| *left <= n)
                        .any(|(_, right)| n <= *right)
            },
            |_| true,
        )
}

fn f1() -> anyhow::Result<()> {
    let data = fs::read_to_string("input1.txt")?;
    let mut res = 0;

    let mut ranges = vec![];

    let mut empty = false;
    for row in data.lines() {
        if row.is_empty() {
            empty = true;
            ranges.sort_unstable_by_key(|&(start, _)| start);
            continue;
        }
        if empty {
            let n: u64 = row.parse().unwrap();
            if contains(&ranges, n) {
                res += 1;
            }
        } else {
            let (left, right) = row.split_once("-").unwrap();
            let (left, right): (u64, u64) = (left.parse().unwrap(), right.parse().unwrap());

            ranges.push((left, right));
        }
    }
    println!("{res}");
    Ok(())
}

fn f2() -> anyhow::Result<()> {
    let data = fs::read_to_string("input2.txt")?;
    let mut res = 0;

    let mut ranges = vec![];

    for row in data.lines() {
        if row.is_empty() {
            ranges.sort_unstable_by_key(|&(start, _)| start);
            let mut current_start = ranges[0].0;
            let mut current_end: u64 = ranges[0].1;

            for &(start, end) in &ranges[1..] {
                if start <= current_end + 1 {
                    current_end = current_end.max(end);
                } else {
                    res += current_end - current_start + 1;
                    current_start = start;
                    current_end = end;
                }
            }
            res += current_end - current_start + 1;
            break;
        }
        let (left, right) = row.split_once("-").unwrap();
        let (left, right): (u64, u64) = (left.parse().unwrap(), right.parse().unwrap());

        ranges.push((left, right));
    }
    println!("{res}");
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
