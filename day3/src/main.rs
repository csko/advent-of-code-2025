use std::{fs, time::SystemTime};

fn maxrange2(r: &[u8], first: usize, last: usize) -> (u8, usize) {
    let mut res = 0u8;
    let mut max_i = 0;
    for (i, c) in r[first..r.len() - last].iter().enumerate() {
        let c = *c;
        if c > res {
            res = c;
            max_i = i;
        }
        if res == b'9' {
            return (b'9', i);
        }
    }
    return (res, max_i);
}

fn solve(row: &[u8], rounds: usize) -> u64 {
    let mut first = 0;
    let mut s = 0;
    for j in 0..rounds {
        let (d1, found_first) = maxrange2(row, first, rounds - j - 1);
        first += found_first + 1;
        s += (d1 - b'0') as u64;
        s *= 10;
    }
    s / 10
}

fn f1() -> anyhow::Result<()> {
    let data = fs::read_to_string("input1.txt")?;
    let mut res = 0;
    for row in data.lines() {
        res += solve(row.as_bytes(), 2);
    }
    println!("{res}");
    Ok(())
}
fn f2() -> anyhow::Result<()> {
    let data = fs::read_to_string("input2.txt")?;
    let mut res = 0;
    for row in data.lines() {
        res += solve(row.as_bytes(), 12);
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
