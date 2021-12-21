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
    let cache = sieve(10000+1)
        .collect::<Vec<_>>();
    let [_n] = parse_line!(1, usize);
    
    for _ in 0.._n {
        let [n] = parse_line!(1, u32);
        let (a, b) = goldbach_partition(n, &cache);
        println!("{} {}", a, b);
    }
}

fn goldbach_partition(num:u32, cache: &Vec<u32>) -> (u32, u32) {
    /*
    an even number which is larger than 2 can be represented as the sum of two prime number
    e.g. 4 => 2 + 2 / 12 => 5 + 7
     */

    // 4 <= n <= 10,000
    // n = even number...
    // find the nearest prime number of n / 2
    let mut index = 0_usize;
    for (i, &prime_num) in cache.iter().enumerate() {
        if 2 * prime_num >= num {
            if 2 * prime_num == num {
                return (prime_num, prime_num)
            }
            index = i;
            break;
        }
    }

    let mut i = index;
    let mut j = index;
    while cache[i] + cache[j] != num {
        let m = cache[i];
        let n = cache[j];
        // println!(">> {} {} / {} {}", n, m, j, i);
        /*
        64 => 37 + 37 (74)
           => 37 + 31 (68)
           => 37 + 29 (66)
           => 37 + 23 (60)
           => 41 + 23 (64)
         */
        match m + n {
            x if x > num => {
                j -= 1;
            },
            x if x < num => {
                i += 1
            },
            _ => return (n, m)
        }
    }
    (cache[j], cache[i])
}