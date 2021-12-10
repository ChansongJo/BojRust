
// https://www.acmicpc.net/source/20352949
// IMPL:parse_line
macro_rules! parse_line {
    (
        $n: expr,
        $t: ty
    ) => ({
        let mut line = String::new();
        // let mut vec = Vec<$t>::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let v = line.split_whitespace()
            .take($n).map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>();

        <[$t; $n]>::try_from(v).ok().unwrap()
    })
}

fn main() {
    let [a, b] = parse_line!(2, u32);
    if a == b {
        println!("==");
    } else if a > b {
        println!(">");
    } else {
        println!("<");
    }
}
