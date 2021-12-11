// IMPL:parse_line
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
fn cmp_two_nums() {
    // 1330
    let [a, b] = parse_line!(2, i32);
    if a == b {
        println!("==");
    } else if a > b {
        println!(">");
    } else {
        println!("<");
    }
}


fn get_test_score() {
    // 9498
    let [a] = parse_line!(1, i32);
    match (a >= 90, a >= 80, a >= 70, a >= 60) {
        (true, _, _, _) => println!("A"),
        (false, true, _, _) => println!("B"),
        (_, false, true, _) => println!("C"),
        (_, _, false, true) => println!("D"),
        (_, _, _, _) => println!("F")
    }
}

fn calc_add_year() {
    // 2753
    let [a] = parse_line!(1, i32);

    let mod_4 = a % 4;
    let mod_100 = a % 100;
    let mod_400 = a % 400;

    match (mod_4 == 0, mod_100 == 0, mod_400 == 0) {
        (true, false, _) => println!("1"),
        (true, true, false) => println!("0"),
        (_, _, true) => println!("1"),
        (_, _, _) => println!("0")
    };
}

fn get_quadrant() {
    // 14681
    let [x] = parse_line!(1, i32);
    let [y] = parse_line!(1, i32);

    match (x > 0, y > 0) {
        (true, true) => println!("1"),
        (false, true) => println!("2"),
        (false, false) => println!("3"),
        (true, false) => println!("4")
    };
}

fn time_calc() {
    // 2884
    let [h, m] = parse_line!(2, i32);
    // -45 minutes...
    // into minutes
    let mut minutes = 60 * h + m - 45;
    if minutes < 0 {
        minutes += 24 * 60;
    }
    // back to hour
    println!("{} {}", minutes / 60, minutes % 60)
}

fn main() {
    time_calc();
}
