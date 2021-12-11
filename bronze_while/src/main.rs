// IMPL:parse_line
use std::{io, io::prelude::*};
use std::convert::TryFrom;

macro_rules! parse_line {
    (
        $n: expr,
        $t: ty
    ) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let v = line.split_whitespace()
            .take($n).map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>();

        <[$t; $n]>::try_from(v).ok().unwrap()
    })
}

macro_rules! parse_line_to_vec {
    (
        $t: ty
    ) => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let v = line.split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>();

        v
    })
}

fn take_while_ab() {
    let [mut a, mut b] = parse_line!(2, u32);
    while a != 0 && b != 0 {
        println!("{}", a + b);
        let vec = parse_line_to_vec!(u32);
        a = vec[0]; b = vec[1];
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
fn take_arbitrary_ab() -> Result<()> {
    for line in io::stdin().lock().lines() {
        let nums = line?.split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        println!("{}", nums[0] + nums[1]);
    }
    Ok(())
}

fn cycle_sum() {
    let [initial] = parse_line!(1, i32);

    let mut num = initial.clone();
    let mut cnt = 0;
    
    while cnt == 0 || num != initial {
        let (a, b) = (num / 10, num % 10);
        num = 10 * b + (a + b) % 10;
        cnt += 1;
    }

    
    println!("{}", cnt)

}

fn main() {
    cycle_sum();
}
