use std::{fs, time::SystemTime};

fn f1() -> anyhow::Result<()> {
    let data = fs::read_to_string("input1.txt")?;
    let mut res = 0u64;
    let mut columns: Vec<Vec<u64>> = vec![];

    let mut first = true;
    for row in data.lines() {
        for (i, col) in row.split_whitespace().enumerate() {
            if first {
                columns.push(Vec::<u64>::new());
            }
            match col {
                "+" => {
                    res += columns[i].iter().sum::<u64>();
                }
                "*" => {
                    res += columns[i].iter().product::<u64>();
                }
                _ => {
                    columns[i].push(col.parse().unwrap());
                }
            }
        }
        first = false;
    }
    println!("{res}");
    Ok(())
}

fn f2() -> anyhow::Result<()> {
    let data = fs::read_to_string("input2.txt")?;
    let mut res = 0;
    let mut table = vec![];

    for row in data.lines() {
        table.push(row.as_bytes());
    }

    let mut nums: Vec<u64> = vec![];
    let mut use_sum = false;
    for i in 0..table[0].len() {
        let mut buf = vec![];
        for j in 0..table.len() {
            let c = table[j][i] as char;
            match c {
                '+' => use_sum = true,
                '*' => use_sum = false,

                other => buf.push(other as u8),
            }
        }
        let buf = String::from_utf8(buf).unwrap();
        let buf = buf.trim();

        if buf.is_empty() {
            // moving on to next block, compute this one
            if use_sum {
                res += nums.iter().sum::<u64>();
            } else {
                res += nums.iter().product::<u64>();
            }
            nums.clear();
        } else {
            nums.push(buf.parse().unwrap());
        }
    }
    if use_sum {
        res += nums.iter().sum::<u64>();
    } else {
        res += nums.iter().product::<u64>();
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
