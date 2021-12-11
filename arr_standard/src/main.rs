// IMPL:parse_line
use std::collections::HashSet;
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

fn get_min_max() {
    parse_line!(1, usize);
    let mut minv = i32::MAX;
    let mut maxv = i32::MIN;
    let vec = parse_line_to_vec!(i32);
    
    for v in vec.iter() {
        minv = *v.min(&minv);
        maxv = *v.max(&maxv);
    }

    println!("{} {}", minv, maxv);
}

fn get_max () {
    let mut maxv = i32::MIN;
    let mut max_idx = 0;
    for i in 0..9 {
        let [x] = parse_line!(1, i32);
        let t = x.max(maxv);
        if t == x {
            maxv = t;
            max_idx = i + 1;
        }

    }
    println!("{}\n{}", maxv, max_idx);
}

fn get_num_digit() {
    let [x] = parse_line!(1, i32);
    let [y] = parse_line!(1, i32);
    let [z] = parse_line!(1, i32);
    let num = x * y * z;

    let mut arr = [0; 10];
    for digit_char in num.to_string().chars() {
        let idx = digit_char.to_string().parse::<usize>().unwrap();
        arr[idx] += 1;
    };

    for count in arr.into_iter() {
        println!("{}", count);
    }
}

fn get_num_mod() {
    let mut arr1 = [0; 10];
    for i in 0..10 {
        let [n] = parse_line!(1, i32);
        arr1[i] = n % 42;
    }
    let hashed: HashSet<i32> = arr1.iter().cloned().collect();
    println!("{}", hashed.len())

    
}

fn fraud_avg_test_score() {
    let [n] = parse_line!(1, f64);
    let vec = parse_line_to_vec!(i32);
    let mut sum = 0;
    let mut max_score: i32 = i32::MIN;
    for num in vec.iter() {
        sum += num;
        max_score = *num.max(&max_score);
    }

    let new_avg: f64 = sum as f64 / max_score as f64 * 100.0;
    println!("{}", new_avg / n)
}

fn ox_quiz() {
    let [n] = parse_line!(1, u8);
    let mut line = String::new();
    for _ in 0..n {
        std::io::stdin().read_line(&mut line).unwrap();

        let mut total_score = 0;
        let mut temp_score = 0;
        for sign in line.chars() {
            if sign.to_string() == "O" {
                temp_score += 1;
                total_score += temp_score;
            } else {
                temp_score = 0;
            }
        }

        total_score += temp_score;
        println!("{}", total_score);
        
        line.clear();
    }
}

fn is_above_avg() {
    let [n] = parse_line!(1, u8);
    for _ in 0..n {
        let vec =  parse_line_to_vec!(f64);
        let n_students = vec[0];
        let total_score: f64 = vec[1..].iter().sum();
        let avg = total_score / n_students;
        // println!("{}", avg);

        let num_above_avg = vec[1..].iter()
            .filter(|x| *x > &avg).count();

        println!("{:.3}%", (num_above_avg as f64 / n_students) * 100.0);
    }
}

fn main() {
    is_above_avg();
}
