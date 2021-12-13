// IMPL:parse_line
fn make_star(num: i32) -> Vec<String> {
    if num == 1 {
        return vec![
            "  *  ".to_string(),
            " * * ".to_string(),
            "*****".to_string(),
        ]
    } else {
        let tri_vec = make_star(num-1);
        let chunk = " ".repeat(tri_vec.len());
        let mut upper = Vec::<String>::new();
        let mut lower = Vec::<String>::new();
        for line in tri_vec.iter() {
            upper.push(format!("{}{}{}", chunk, *line, chunk));
            lower.push(format!("{} {}", *line, line));
        }
        upper.append(&mut lower);
        return upper
    }
}

fn star_11() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n = line.trim().parse::<u32>().expect("error");
    let real_n = ((n / 3) as f64).log2() as i32 + 1;
    println!("{}", make_star(real_n).join("\n"));
}

fn main() {
    star_11();

}
