// IMPL:parse_line
use std::convert::{TryFrom, TryInto};

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

fn find_revenue_point() {
    let [a , b , c] = parse_line!(3, i32);
    if (c - b) <= 0 {
        println!("-1")
    } else {
        let res = a / (c - b) + 1;
        println!("{}", res)
    }
}

fn honeycomb() {
    let [a] = parse_line!(1, i32);
    // logic
    // find out "a" is in what degree in HC
    // 1, 7, 19, 37, 61
    //   6  12  18  24
    if a == 1 {
        println!("{}", 1)
    } else {
        let mut degree = 0;
        let mut sum = 1;
        while a > sum {
            degree += 1;
            sum += 6 * degree;
        }
        println!("{}", degree+1)
    }
}


fn make_star(num: i32) -> Vec<String> {
    if num == 0 {
        return vec![String::from("*")]
    } else {
        let rectangle = make_star(num - 1);
        let mut upper = Vec::<String>::new();
        let mut lower = Vec::<String>::new();
        for line in rectangle.iter() {
            upper.push(line.repeat(3));
            lower.push(format!("{}{}{}", line, " ".repeat(rectangle.len()), line));
        };

        let mut copied_upper = upper.clone();
        upper.append(&mut lower);
        upper.append(&mut copied_upper);
        return upper
    }
}

fn star_10() {
    let [mut n] = parse_line!(1, f64);
    n = n.log(3.0).round();
    let vec = make_star(n as i32);
    println!("{}", vec.join("\n"))

}


fn find_fraction() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<f64>().unwrap();
    // 근의 공식
    // x = -b +- (b.pow(2) - 4ac).sqrt() / 2

    let deg = ((-1.0 + ((1.0 + (8.0 * n))).sqrt()) / 2.0).ceil() as i32;
    let sum = deg * (deg + 1) / 2;
    let diff = sum - n as i32;
    let i = deg - diff;
    let j = 1 + diff;

    if deg % 2 == 1 {
        println!("{}/{}", j, i);
    } else {
        println!("{}/{}", i, j);
    }
}

fn climbing_snail() {
    //  1 <= down < up <= length 1_000_000_000
    let [up, down, length] = parse_line!(3, u32);
    if up == length {
        println!("1");
    } else {
        let diff = up - down;
        let naive_date = ((length - up) / diff).max(1);

        // run brute hear
        let mut date = naive_date;
        let mut sum = naive_date * (up - down);
        while sum < length {
            sum += up;
            date += 1;
        }
        println!("{}", date);
    }
}

use std::iter;
fn really_large_num_sum() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut vec = line.split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    
    vec.sort_by(|a, b| a.len().cmp(&b.len()));
    
    let mut _a = vec[0].to_owned(); // short
    let mut _b = vec[1].to_owned(); // long

    let mut res = vec![0; _b.len() + 1];
    let iter = _b.chars().rev()
        .zip(_a.chars().rev()
            .chain(iter::repeat('0')));
    let mut additional = 0;
    for (a, b) in iter {
        let summed = a.to_digit(10).unwrap() 
            + b.to_digit(10).unwrap()
            + additional;
        
        additional = summed / 10;
        res.push(summed % 10);
    };
    if additional == 1 {
        res.push(additional);
    }

    for digit in res.iter().rev() {
        print!("{}", digit)
    }
}

use std::io::Write;

fn acm_hotel() {
    let [_n] = parse_line!(1, u32);
    let mut d = Vec::new();
    for _ in 0.._n {
        let [h, w, n] = parse_line!(3, u32);
        let mut room_number = n / h;
        let mut floor_number = n % h;
        if floor_number == 0 {
            floor_number = h
        } else {
            room_number += 1
        }

        write!(d, "{}{:02}\n", floor_number, room_number).unwrap();
    }
    print!("{}", std::str::from_utf8(&d).unwrap())
}

fn weird_apartment() {
    // build apart
    const MAX_NUM: usize = 14 + 1;
    let mut apartment = [[0_u32; MAX_NUM]; MAX_NUM];
    for i in 0..MAX_NUM {
        for j in 0..MAX_NUM {
            if i == 0 {
                apartment[i][j] = (j + 1) as u32;
            } else if j == 0 {
                apartment[i][j] = 1;
            } else {
                apartment[i][j] = apartment[i-1][j] + apartment[i][j-1];
            }
        }
    }
    let [_n] = parse_line!(1, u32);
    for _ in 0.._n {
        let [k] = parse_line!(1, usize);
        let [n] = parse_line!(1, usize);
        // 0, 3 => 3  | 1, 2, 3
        // 1, 3 => 6  | 1, 3, 6
        // 2, 3 => 10 | 1, 4, 10
        println!("{}", apartment[k][n - 1]);
    }
}

fn sugar_delivery() {
    let [amount] = parse_line!(1, i32);
    // try with five
    // limit (3 ≤ N ≤ 5000)
    let mut bag = -1;
    if amount % 5 == 0 {
        bag = amount / 5;
    } else {
        for i in (0..=(amount / 5)).rev() {
            let rest = (amount - (5 * i)) % 3;
            if rest == 0 {
                bag = i + (amount - (5 * i)) / 3;
                break;
            }
        }
    }

    println!("{}", bag)
}

fn fly_me_to_the_alpha_centauri() {
    let [_n] = parse_line!(1, usize);
    for _ in 0.._n {
        let [a, b] = parse_line!(2, f64);
        let distance = b - a;

        // n(n+1) + n(n-1)
        // n^2...
        
        // 3 -> 3
        // how? : 1, 1, 1
        // 4 -> 3
        // how? : 1, 2, 1 
        // 5 -> 4
        // how?: 1, 2, 1, 1

        // 6 -> 4
        // how?: 1, 2, 2, 1

        // 7 -> 5
        // how?: 1, 2, 3, 2, 1 => 2 * (2 ** 2)
        // 4(2) -> 3
        // 9(3) -> 5
        // 16(4) -> 7 1, 2, 3, 4, 3, 2, 1

        let res = distance.sqrt() + 1.0;
        println!("{}", res as u32);
    }

}

fn main() {
    fly_me_to_the_alpha_centauri();
}
