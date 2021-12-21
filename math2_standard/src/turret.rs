// IMPL:parse_line
use std::{convert::{TryFrom, TryInto}, ops::RangeToInclusive};

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

fn main() {
    let [_n] = parse_line!(1, u32);
    for _ in 0.._n {
        let [x1, y1, r1, 
             x2, y2, r2] = parse_line!(6, i32);

        // calc center distance 
        let distance = (x1 - x2).pow(2) + (y1 - y2).pow(2);

        match distance == 0 {
            true => {
                if r1 == r2 {
                    println!("-1");
                } else {
                    println!("0");
                }
            },
            false => {
                if distance < (r1 + r2).pow(2) && distance > (r1 - r2).pow(2) {
                    // 교차
                    println!("2");
                } else if distance == (r1 + r2).pow(2) {
                    // 외접
                    println!("1");
                } else if (r1-r2).pow(2) == distance {
                    // 내접
                    println!("1");
                } else {
                    println!("0");
                }
            }
        }

    }
}
