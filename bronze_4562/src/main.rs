use std::io;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

const NO: &str = "NO BRAINS";
const YES: &str = "MMM BRAINS";

pub fn read_single_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

pub fn main() -> Result<()> {
    // indicator n: a number of following datasets
    let n = read_single_line().trim().parse::<i32>().unwrap();
    for _ in 0..n {
        // do some logic
        let res = read_single_line()
            .trim()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .reduce(|a, b| a - b)
            .unwrap();

        if res >= 0 {
            println!("{}", YES);
        } else {
            println!("{}", NO);
        }
    }
    Ok(())
}
