use std::{fs, time::SystemTime};

fn f1() -> anyhow::Result<()> {
    let data = String::from_utf8(fs::read("input1.txt")?)?;
    let mut res = 0;
    for row in data.split("\r\n") {
        let values: Vec<_> = row.split(" ").collect();
    }
    Ok(())
}

fn f2() -> anyhow::Result<()> {
    let data = String::from_utf8(fs::read("input1.txt")?)?;
    let mut res = 0;
    for row in data.split("\r\n") {
        let values: Vec<_> = row.split(" ").collect();
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
