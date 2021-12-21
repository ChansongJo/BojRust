// IMPL:parse_line
use std::{convert::{TryFrom}};

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
    let arr = parse_line!(8, i32);

    let mut mode = "undef";
    for i in 1..arr.len() {
        match (mode, arr[i] - arr[i-1] > 0) {
            ("undef", true) => {
                mode = "ascending";
            },
            ("undef", false) => {
                mode = "descending";
            },
            ("ascending", false)|("descending", true) => {
                mode = "mixed";
                break
            },
            (_, _) => continue
        }
    }
    println!("{}", mode)
}
