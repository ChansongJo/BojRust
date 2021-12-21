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

fn main() {
    let cache = sieve(123456 * 2)
        .collect::<Vec<_>>();
    'main: while true {
        let [n] = parse_line!(1, u32);
        if n == 0 {
            break 'main;
        }

        let num = bertrand_theorem(n, &cache);
        println!("{}", num);
    };
}

fn bertrand_theorem(n:u32, cache: &Vec<u32>) -> usize {
    /*
    There is an at least one prime number
    between arbitrary natural number n and 2n

    e.g. 10 - 20 => 11, 13, 17, 19
    e.g. 14 - 28 => 17, 19, 23
     */

    cache.iter()
        .filter(|v| (&n < *v) && (*v <= &(2 * n)))
        .count()
}