// IMPL:parse_line
use std::convert::{TryFrom, TryInto};
use std::io::{self,Write,BufWriter};

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

fn is_hansu(num: i32) -> bool {
    let mut m = num;
    let mut common_diff: i32 = i32::MAX;
    while m > 9 {
        let ones = m % 10;
        let tens = (m / 10) % 10;
        if common_diff != i32::MAX && common_diff != ones - tens {
            return false;
        };
        // move to next posit
        m /= 10;
        common_diff = ones - tens
    }
    true
}


fn hansu(length: i32) {
    let mut cnt = 0;
    for i in 1..=length {
        if is_hansu(i) {
            cnt += 1;
        }
    }
    println!("{}", cnt)
}

fn main() {
    let [num] = parse_line!(1, i32);
    hansu(num);
}
