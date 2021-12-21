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


fn factorial(n: i32) -> i32 {
    if n < 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

fn fibo(n: i32) -> i32 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => return fibo(n-1) + fibo(n-2)
    }
}

/* 
def hanoi(number_of_disks_to_move, from_, to_, via_):
    if number_of_disks_to_move == 1:
        print(from_, "->", to_)
    else:
        hanoi(number_of_disks_to_move-1, from_, via_, to_)
        print(from_, "->", to_)
        hanoi(number_of_disks_to_move-1, via_, to_, from_)
*/

fn hanoi(n: i32, from: i32, to: i32, via: i32, buf: &mut Vec<String>) {
    if n == 1 {
        buf.push(format!("{} {}", from, to));
    } else {
        hanoi(n - 1, from, via, to, buf);
        buf.push(format!("{} {}", from, to));
        hanoi(n - 1, via, to, from, buf);
    }
}

fn main() {
    let [a] = parse_line!(1, i32);
    let mut buffer: Vec<String> = Vec::new();
    println!("{}", 2_i32.pow(a as u32) - 1);
    hanoi(a, 1, 3, 2, &mut buffer);
    println!("{}", buffer.join("\n"))
}
