use std::cmp::min;

// https://www.acmicpc.net/source/20352949
// IMPL:parse_line
#[macro_use]
mod macros {
    macro_rules! parse_line {
        ($($t: ty),+) => ({
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            // what about 1 2 3 ... ???
            // or none digit values...
            ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
        })
    }
}

pub fn search(upper: u32, v: u32) -> u32 {
    // ...
    let sqrtv = (v as f64).sqrt() as u32;
    let mut sum: u32 = 0;
    for i in 1..sqrtv {
        sum += min(upper, v / i);
    }

    let midend = min(upper, v / (sqrtv - 1));
    for i in (1..=midend).rev() {
        sum += min(upper, v / i);
    }

    sum - ((sqrtv - 1) * midend)
}

fn main() {
    let n = parse_line!(u32);
    let k = parse_line!(u32);

    let (mut s, mut e) = (0, n * n); // f(0) < k <= f(n*n) verified
    while s + 1 < e {
        let m = (s + e) / 2;
        if k <= search(n, m) {
            e = m;
        } else {
            s = m;
        }
    }

    println!("{}", e);
}
