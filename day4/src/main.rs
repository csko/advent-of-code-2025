use std::{fs, time::SystemTime};

// const M: i64 = 10;
// const N: i64 = 10;
const M: i64 = 140;
const N: i64 = 140;

fn get(grid: &Vec<Vec<char>>, i: i64, j: i64) -> bool {
    i >= 0 && j >= 0 && i < N && j < M && grid[i as usize][j as usize] == '@'
}

fn n_count(grid: &Vec<Vec<char>>, i: i64, j: i64) -> usize {
    let mut n = 0;
    for (mut ni, mut nj) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        ni += i;
        nj += j;

        if get(grid, ni, nj) {
            n += 1;
        }
    }
    return n;
}

fn f1() -> anyhow::Result<()> {
    let data = fs::read_to_string("input1.txt")?;
    let mut res: i64 = 0;
    let mut grid: Vec<Vec<char>> = vec![];

    for row in data.lines() {
        let mut grid_row = vec![];
        for c in row.chars() {
            grid_row.push(c);
        }
        grid.push(grid_row);
    }

    for i in 0..N {
        for j in 0..M {
            if grid[i as usize][j as usize] == '@' && n_count(&grid, i, j) < 4 {
                res += 1;
            }
        }
    }

    println!("{res}");
    Ok(())
}

fn f2() -> anyhow::Result<()> {
    let data = fs::read_to_string("input2.txt")?;
    let mut res: i64 = 0;
    let mut grid: Vec<Vec<char>> = vec![];

    for row in data.lines() {
        let mut grid_row = vec![];
        for c in row.chars() {
            grid_row.push(c);
        }
        grid.push(grid_row);
    }

    loop {
        let mut this_round = 0;
        for i in 0..N {
            for j in 0..M {
                if grid[i as usize][j as usize] == '@' && n_count(&grid, i, j) < 4 {
                    this_round += 1;
                    grid[i as usize][j as usize] = 'x';
                }
            }
        }
        if this_round == 0 {
            res += this_round;
            break;
        }
    }

    for i in 0..N {
        for j in 0..M {
            if grid[i as usize][j as usize] == 'x' {
                res += 1;
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
