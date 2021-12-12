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

fn d(num: usize) -> usize {
    let mut sum = num;
    let mut m = num;
    while m > 0 {
        sum += m % 10;
        m /= 10;
    }
    sum
}

fn self_number() {
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut buff = io::BufWriter::new(lock);

    const LENGTH: usize = 10000;
    let mut bag = [1; LENGTH + 1];
    for i in 1..=LENGTH {
        if bag[i] == 1 {
            buff.write_fmt(format_args!("{}\n", i)).expect("print error");
        };

        let temp = d(i);
        if temp <= LENGTH {
            bag[d(i)] = 0;
        }
    }
}

fn main() {
    self_number()
}
