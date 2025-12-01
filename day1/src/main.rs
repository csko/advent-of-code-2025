use std::{fs, time::SystemTime};

const START_POS: i32 = 50;
const WRAP_SIZE: i32 = 100;

fn f1() -> anyhow::Result<()> {
    let data = fs::read_to_string("input1.txt")?;
    let mut res: u32 = 0;
    let mut pos: i32 = START_POS;
    for row in data.lines() {
        let dir = row.as_bytes()[0];
        let n: i32 = row[1..].parse().unwrap();
        if dir == b'L' {
            pos -= n;
        } else {
            pos += n;
        }
        if pos < 0 {
            pos += WRAP_SIZE;
        }
        pos %= WRAP_SIZE;
        if pos == 0 {
            res += 1;
        }
    }
    println!("{res}");
    Ok(())
}

fn f2() -> anyhow::Result<()> {
    let data = fs::read_to_string("input2.txt")?;
    let mut res: u32 = 0;
    let mut pos: i32 = START_POS;
    for row in data.lines() {
        let dir = row.as_bytes()[0];
        let n: i32 = row[1..].parse().unwrap();
        let (full_wraps, n) = (n / WRAP_SIZE, n % WRAP_SIZE);
        res += full_wraps as u32;

        if dir == b'L' {
            if pos > 0 && pos - n <= 0 {
                res += 1;
            }
            pos -= n;
            if pos < 0 {
                pos += WRAP_SIZE;
            }
        } else {
            pos += n;
            if pos >= WRAP_SIZE {
                res += 1;
                pos %= WRAP_SIZE;
            }
        }
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
