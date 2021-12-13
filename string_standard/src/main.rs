use std::io;
use io::Write;

fn ord() {
    let mut line = String::with_capacity(1);
    std::io::stdin().read_line(&mut line).unwrap();
    let stdout = std::io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());

    let n = line.trim().parse::<usize>().expect("error");
    line.clear();

    let mut arr = [0; 10001];
    for _ in 0..n {
        std::io::stdin().read_line(&mut line).unwrap();
        let num = line.trim().parse::<u32>().expect("error");
        // need some dynamic sorting...
        // try with dumb way...
        arr[num as usize] += 1;
        line.clear();
    }

    for (i, n) in arr.iter().enumerate()
        .filter(|(i, _)| i > &0) {
        for _ in 0..*n {
            writeln!(out, "{}", &i).unwrap();
        }
    }
}

fn main() {
    ord();
}
