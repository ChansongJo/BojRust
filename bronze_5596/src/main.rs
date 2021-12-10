use std::{cmp, io, io::prelude::*};
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;


pub fn parse(line: String) -> i32 {
    line.trim()
    .split_whitespace()
    .map(|num| num.parse::<i32>().unwrap())
    .reduce(|a, b| a + b)
    .unwrap()
}


pub fn main() -> Result<()> {
    // line 1
    let mut res = -1;
    for _ in 0..2 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let score = parse(line);
        res = cmp::max(res, score);
    }

    println!("{}", res);
    Ok(())
}

pub fn __main() -> Result<()> {
    let mut res = 0;
    for line in io::stdin().lock().lines() {
        let score = parse(line?);
        if score > res {
            res = score;
        }
    }
    println!("{}", res);
    Ok(())
}