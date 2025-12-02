use std::{fs, time::SystemTime};

fn invalid_id(inp: &[u8]) -> bool {
    let len = inp.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    inp[0..half] == inp[half..]
}

fn invalid_id2(inp: &[u8]) -> bool {
    let len = inp.len();
    for times in 2..len + 1 {
        if len % times != 0 {
            continue;
        }
        let half = len / times;
        let mut found = true;
        for j in 1..times {
            if inp[0..half] != inp[half * j..half * (j + 1)] {
                found = false;
                break;
            }
        }
        if found {
            return true;
        }
    }
    false
}

fn f1() -> anyhow::Result<()> {
    let data = fs::read_to_string("input1.txt")?;
    let mut res: i64 = 0;
    for row in data.lines() {
        for range in row.split(",") {
            let (first, last) = range.split_once("-").unwrap();
            let (first, last): (i64, i64) = (first.parse().unwrap(), last.parse().unwrap());
            for id in first..last + 1 {
                if invalid_id(&format!("{}", id).as_bytes()) {
                    res += id as i64;
                }
            }
        }
    }
    println!("{res}");
    Ok(())
}

fn f2() -> anyhow::Result<()> {
    let data = fs::read_to_string("input2.txt")?;
    let mut res: i64 = 0;
    for row in data.lines() {
        for range in row.split(",") {
            let (first, last) = range.split_once("-").unwrap();
            let (first, last): (i64, i64) = (first.parse().unwrap(), last.parse().unwrap());
            for id in first..last + 1 {
                if invalid_id2(&format!("{}", id).as_bytes()) {
                    res += id as i64;
                }
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
