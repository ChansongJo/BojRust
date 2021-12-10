use std;

fn read_from_stdin() -> String {
    let mut line = String::new();
    let stdin = std::io::stdin();
    let res = stdin.read_line(&mut line);
    match res {
        Ok(_) => return line,
        Err(_) => return String::from(r#"error occured"#),
    }
}

fn main_dumb() {
    let x = read_from_stdin().trim().parse::<i32>().unwrap();
    let y = read_from_stdin().trim().parse::<i32>().unwrap();

    let mut rest = y.clone();
    let hunds = rest / 100;
    rest = rest % 100;
    let tens = rest / 10;
    rest = rest % 10;
    let ones = rest;

    println!("{}", x * ones);
    println!("{}", x * tens);
    println!("{}", x * hunds);
    println!("{}", x * y);

}

fn main() {
    let x = read_from_stdin().trim().parse::<u32>().unwrap();
    let y = read_from_stdin().trim().to_owned();

    for digit in y.chars().rev() {
        let multiplied: u32 = x * digit.to_digit(10).unwrap();
        println!("{}", multiplied)
    }
    println!("{}", x * y.parse::<u32>().unwrap())

}
