use std::io;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn read_single_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

pub fn main() -> Result<()> {
    // indicator n: a number of following datasets
    let input_meta: Vec<usize> = read_single_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    //let word_count = input_meta[0];
    let line_limit = input_meta[1];
    
    let mut buffer = String::with_capacity(line_limit);
    let mut buffer_word_count = 0;
    for word in read_single_line().trim().split_whitespace() {
        //println!("{} {} {} {}", buffer.len(), word.len(), line_limit, buffer_word_count);
        if  (buffer.len() + word.len()) < (line_limit + buffer_word_count) {
            // push to buffer
            if buffer.len() != 0 {
                buffer.insert(buffer.len(), ' ');
            }
        } else {
            // flush buffer
            println!("{}", buffer);
            buffer.clear();
            buffer_word_count = 0;
        }
        buffer.insert_str(buffer.len(), word);
        buffer_word_count += 1;
    }
    println!("{}", buffer);

    Ok(())
}