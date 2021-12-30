use std::{convert::{TryFrom, TryInto}, hash};
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

fn sort1() {
    let [n] = parse_line!(1, i32);
    let mut vec = Vec::<i32>::new();
    for i in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let num = line.trim().parse::<i32>().unwrap();
        vec.push(num);
        line.clear();
    }
    vec.sort();

    let buf = vec.iter().map(|x| x.to_string())
        .collect::<Vec<String>>().join("\n");
    
    println!("{}", buf)
}

use std::cmp::{Eq, Ord, Reverse, Ordering};
use std::collections::{BinaryHeap, HashMap};

fn statistics() {
    let [n] = parse_line!(1, i32);
    let mut vec = Vec::<i32>::new();
    let mut counter = HashMap::<i32, i32>::new();

    let mut sum = 0;
    for i in 0..n {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let num = line.trim().parse::<i32>().unwrap();
        sum += num;

        *counter.entry(num).or_insert(0) += 1;
        vec.push(num);
    }
    vec.sort();

    let mut cnt_vec = counter
        .iter()
        .collect::<Vec<_>>();
    
    cnt_vec.sort_by(|a, b| 
        match (a.1.cmp(&b.1), a.0.cmp(&b.0)) {
            (Ordering::Equal, Ordering::Less) => Ordering::Less,
            (Ordering::Equal, _) => Ordering::Greater,
            (Ordering::Greater, _) => Ordering::Less,
            (Ordering::Less, _) => Ordering::Greater
    });

    let mut val = 0;
    if cnt_vec.len() == 1 {
        val = *cnt_vec[0].0;
    } else if *cnt_vec[0].1 == *cnt_vec[1].1 {
        val = *cnt_vec[1].0
    } else {
        val = *cnt_vec[0].0
    }

    let max = vec[vec.len() - 1];
    let min = vec[0];
    let median = vec[vec.len() / 2];
    let avg = (sum as f64 / vec.len() as f64).round() as i32;

    println!("{}\n{}\n{}\n{}", avg, median, val, max-min)
}



fn sort_inside() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.truncate(line.len() - 1);
    let mut vec: Vec<u32> = line.chars()
        .map(|n| n.to_digit(10).unwrap())
        .collect();

    vec.sort_by(|a, b| b.cmp(&a));
    let msg = vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("{}", msg);
}



#[derive(Eq)]
pub struct Person {
    age: i32,
    name: String,
    order: usize
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.age.cmp(&other.age), self.order.cmp(&other.order)) {
            (Ordering::Greater, _) => Ordering::Greater,
            (Ordering::Equal, x) => x,
            (Ordering::Less, _) => Ordering::Less,
        }
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

fn sort_point() {
    let [n] = parse_line!(1, usize);
    let mut line = String::new();

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let item = Person {
            age: iter.next().unwrap().parse::<i32>().unwrap(),
            name: iter.next().unwrap().to_string(),
            order: i
        };

        heap.push(item);
        line.clear();
    }
    let vec = heap.into_sorted_vec();
    let msg: String = vec.iter()
        .map(|word| format!("{} {}\n", word.age, word.name))
        .reduce(|a, b| a + &b)
        .unwrap();
    println!("{}", msg);
}


use std::io::{self, Read};

fn point_zip() {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut xs: Vec<(usize, i32)> = lines
        .next()
        .unwrap()
        .split(' ')
        .take(n)
        .map(|n| n.parse().unwrap())
        .enumerate()
        // .map(|(i, j)| (i as i32, j))
        .collect();

        xs.sort_unstable_by_key(|x| x.1);
        
    buf.clear();
    let mut unique_count = 0;
    let mut result = vec![String::new(); n];
    for i in 1..n {
        if xs[i].1 != xs[i - 1].1 {
            unique_count += 1;
        }
        result[xs[i].0] = format!("{}", unique_count);
    }

    println!("{}", result.join(" "));
}


fn main() {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut xs: Vec<(usize, i32)> = lines
        .next()
        .unwrap()
        .split(' ')
        .take(n)
        .map(|n| n.parse().unwrap())
        .enumerate()
        .collect();

    buf.clear();
    xs.sort_unstable_by_key(|x| x.1);

    let mut unique_count = 0;
    let mut result = vec![String::new(); n];
    for i in 1..n {
        if xs[i].1 != xs[i - 1].1 {
            unique_count += 1;
        }
        result[xs[i].0] = format!("{}", unique_count);
    }

    println!("{}", result.join(""));
}

fn check_num() {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let answer = lines.next().unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| x.pow(2))
        .sum::<u32>() % 10;
    
    println!("{}", answer)
}