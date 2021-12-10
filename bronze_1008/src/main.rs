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
    let result = line.trim()
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .reduce(|a, b| a / b)
        .unwrap();
    
    println!("{}", result);
}