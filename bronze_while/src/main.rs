// IMPL:parse_line
use std::io;
use std::io::{BufWriter, Write};
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

fn gugu() {
    // 2739
    let [dan] = parse_line!(1, u32);

    for i in 1..10 {
        println!("{} * {} = {}", dan, i, dan * (i as u32))
    }
}

fn multi_a_b_sum() {
    // 10950, 15552, 11021, 11022
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let [n_cases] = parse_line!(1, usize);
    for i in 1..=n_cases {
        let [a, b] = parse_line!(2, i32);
        let res = format!("Case #{}: {} + {} = {}", i, a, b, a+b);
        writeln!(out, "{}", res).unwrap();
    }
}

fn sum() {
    // 8393
    let [max_num] = parse_line!(1, u32);

    println!("{}", max_num * (max_num + 1) / 2)
}

fn printn() {
    // 2741
    let [n] = parse_line!(1, u32);
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for i in 1..=n {
        writeln!(out, "{}", i).unwrap();

    }
}

fn printn_rev() {
    // 2742
    let [n] = parse_line!(1, u32);
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for i in (1..=n).rev() {
        writeln!(out, "{}", i).unwrap();

    }
}

fn star_print() {
    // https://users.rust-lang.org/t/fill-string-with-repeated-character/1121/8
    // 2439
    let [max_num] = parse_line!(1, usize);
    for i in 1..=max_num {
        let msg = format!("{: >1$}", String::from("*").repeat(i), max_num);
        println!("{}", msg)
    }
}

fn smaller_than_x() {
    let [_, x] = parse_line!(2, u32);
    let nums = parse_line_to_vec!(u32)
        .into_iter().filter(|v| *v < x )
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    println!("{}", nums.join(" "));
}

fn main() {
    smaller_than_x();
}
