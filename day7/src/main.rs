// use std::collections::HashMap;
use std::iter::zip;
use std::{fs, time::SystemTime};

use fxhash::FxHashMap as HashMap;

fn f1() -> anyhow::Result<()> {
    let data = fs::read_to_string("input1.txt")?;
    let mut res = 0u64;

    let (repr, rest) = data.split_once("\r\n").unwrap();
    let repr = repr.replace("S", "|");
    let mut repr: Vec<char> = repr.as_str().chars().collect();

    for row in rest.lines() {
        let mut next_repr = repr.clone();
        for (i, (c1, c2)) in zip(repr.iter(), row.chars()).enumerate() {
            if *c1 == '|' {
                if c2 == '^' {
                    res += 1;
                    next_repr[i - 1] = '|';
                    next_repr[i] = '.';
                    next_repr[i + 1] = '|';
                } else {
                    next_repr[i] = '|';
                }
            } else {
                if next_repr[i] != '|' {
                    next_repr[i] = '.';
                }
            }
        }
        repr = next_repr;
    }
    println!("{res}");
    Ok(())
}

fn solve(
    pos: usize,
    row_id: usize,
    rows: &Vec<Vec<char>>,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    match memo.get(&(pos, row_id)) {
        Some(res) => return *res,
        None => {
            let res;
            if row_id == rows.len() - 1 {
                if rows[row_id][pos] == '^' {
                    res = 2;
                } else {
                    res = 1;
                }
            } else {
                if rows[row_id][pos] == '^' {
                    res = solve(pos - 1, row_id + 1, rows, memo)
                        + solve(pos + 1, row_id + 1, rows, memo);
                } else {
                    res = solve(pos, row_id + 1, rows, memo);
                }
            }
            memo.insert((pos, row_id), res);
            return res;
        }
    }
}

fn f2() -> anyhow::Result<()> {
    let data = fs::read_to_string("input2.txt")?;
    let res;

    let (repr, rest) = data.split_once("\r\n").unwrap();
    let mut rows = vec![];
    for (i, row) in rest.lines().enumerate() {
        if i % 2 == 0 {
            continue;
        }
        rows.push(row.chars().collect());
    }

    let pos = repr.find("S").unwrap();
    let mut memo: HashMap<(usize, usize), u64> = HashMap::default();
    res = solve(pos, 0, &rows, &mut memo);

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
