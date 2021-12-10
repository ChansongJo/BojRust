use std;

fn read_from_stdin() -> String {
    let mut line = String::new();
    let stdin = std::io::stdin();
    let res = stdin.read_line(&mut line);
    match res {
        Ok(_) => return line,
        Err(_) => return String::from(r#"error occured"#)
    }
}

fn main() {
    let line = read_from_stdin();
    let result: Vec<i32> = line.trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    
    let x = result[0];
    let y = result[1];
    println!("{}", x + y);
    println!("{}", x - y);
    println!("{}", x * y);
    println!("{}", x / y);
    println!("{}", x % y);
}