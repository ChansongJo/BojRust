use std::fmt::Write;
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut xs: Vec<(i32, i32)> = lines
        .next()
        .unwrap()
        .split(' ')
        .take(n)
        .map(|n| n.parse().unwrap())
        .enumerate()
        // .map(|(i, j)| (i as i32, j))
        .collect();

    buf.clear();
    xs.sort_unstable_by_key(|x| x.1);

    let mut unique_count = 0;
    let mut result = vec![0; n];
    for i in 1..n {
        if xs[i].1 != xs[i - 1].1 {
            unique_count += 1;
        }
        result[xs[i].0] = unique_count;
    }

    let msg = result
        .map(|x| format!("{} ", x))
        .reduce(|a, b| a + b);

    println!("{}", result);
}