use std::io;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn read_single_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

const WIN: &str = "MS";
const LOSS: &str = "TK";
const DRAW: &str = "?";

pub fn main() -> Result<()> {
    // indicator n: a number of following datasets
    let input: Vec<String> = read_single_line()
        .trim()
        .split_whitespace()
        .map(|x| String::from(x))
        .collect();

    // case 1 [R S R P] -> ? (draw)
    // case 2 [R R S S] -> MS (win)
    // case 3 [P P S R] -> TK (win)
    let mut matches: Vec<&str> = vec![];
    for p1_hand in &input[..2] {
        for p2_hand in &input[2..] {
            let temp_res = match (p1_hand.as_str(), p2_hand.as_str()) {
                ("R", "S") => "win",
                ("P", "S") => "loss",
                ("R", "P") => "loss",
                ("S", "P") => "win",
                ("P", "R") => "win",
                ("S", "R") => "loss",
                (_, _) => "draw",
            };
            matches.push(temp_res);
        }
    }

    match matches.as_slice() {
        ["win", "win", _, _] => println!("{}", WIN), // p1 LH must win
        [_, _, "win", "win"] => println!("{}", WIN), // p1 RH must win
        ["loss", _, "loss", _] => println!("{}", LOSS), // p2 LH must win
        [_, "loss", _, "loss"] => println!("{}", LOSS), // p2 RH must win
        _ => println!("{}", DRAW),
    };
    Ok(())
}
