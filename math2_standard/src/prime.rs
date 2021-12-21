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
    let [a, b] = parse_line!(2, u32);
    let primes = sieve(b);
    let msg = primes.filter(|x| x >= &a)
        .map(|x| x.to_string())
        .collect::<Vec<String>>().join("\n");

    println!("{}", msg);
}

fn sieve(n:u32) -> std::iter::Chain<
        std::ops::RangeInclusive<u32>, 
        impl Iterator<Item = u32>
    > {
    // Set of odd numbers. 3, 5, 7, 9 and so on.
    let len = (n - 1) / 2;
    let mut odds = vec![true;len as usize];
    let mut i = 0;
    while i < len {
        if odds[i as usize] {
            let prime = i * 2 + 3;
            let mut j = i + prime;
            while j < len {
                odds[j as usize] = false;
                j = j + prime;
            }
        }
        i = i + 1;
    }
    let _v = odds.into_iter().enumerate()
        .filter(|(_, x)| *x)
        .map(|(i, _)| (i * 2 + 3) as u32);

    (2..=2_u32).chain(_v)
}