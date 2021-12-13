// IMPL:parse_line
use std::convert::{TryFrom, TryInto};
use std::io::{self,Write,BufWriter};
use std::collections::HashMap;

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

fn ascii_to_u32() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("stdin error");
    for letter in line.trim().chars() {
        println!("{}", letter as u32);
    }
}

fn add_two_num() {
    parse_line!(1, u32);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("stdin error");
    let sum = line.trim().chars()
        .map(|x| x.to_string().parse::<u32>().expect("parse error"))
        .reduce(|a, b| a + b).unwrap();
    
    println!("{}", sum)
}

fn find_alphabet() {
    let mut res: [i32; 26] = [-1; 26];

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin error");
    for (i, code) in line.trim().chars().map(|x| x as i32).enumerate() {
        let idx = (code - 97) as usize;
        if res[idx] == -1 {
            res[idx] = i as i32;
        }
    }
    println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}

fn repeat_char() {
    let [n] = parse_line!(1, u32);    
    let mut line = String::new();

    for _ in 0..n {
        io::stdin().read_line(&mut line).expect("stdin error");
        let vec = line.trim().split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let repeat_cnt = &vec[0].parse::<usize>().expect("parse error");
        let word = &vec[1];

        for c in word.chars() {
            print!("{}", c.to_string().repeat(*repeat_cnt));
        }
        print!("\n");


        line.clear();
    }
}

fn get_max_cnt_alphabet() {
    let mut alphabets: [i32; 26] = [0; 26];
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin error");
    line.truncate(line.trim().len());
    for c in line.to_ascii_lowercase().chars() {
        let idx: usize = c as usize - 97;
        alphabets[idx] += 1;
    }

    let mut max_cnt = -1;
    let mut max_idx: i32 = -1;
    for (i, alphabet_cnt) in alphabets.iter().enumerate() {
        match alphabet_cnt != &max_cnt {
            // case 1 alphabet_cnt != max_cnt
            true => {
                match alphabet_cnt > &max_cnt {
                    true => {
                        max_cnt = *alphabet_cnt;
                        max_idx = i as i32;
                    },
                    false => {
                        continue;   
                    }
                } 
            },
            // alphabet_cnt == max_cnt
            false => {
                max_idx = -1;
            }
        };
    }
    if max_idx >= 0 {
        println!("{}", char::from_u32(max_idx as u32 + 65).expect("Invalid"));
    } else {
        println!("?")
    }
}

fn get_word_cnt() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin error");
    let cnt = line.split_whitespace().count();
    println!("{}", cnt)

}

fn weird_word_cmp() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin error");
    let cnt = line.split_whitespace()
        .map(|x| x.chars().rev().collect::<String>())
        .map(|x| x.parse::<u32>().expect("Invalid parse error"))
        .reduce(|a, b| a.max(b)).expect("Invalid reduce");
    
    println!("{}", cnt)
}

fn grandmas_old_phone() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin error");
    // lets get functional
    let res = line.trim().chars()
        .map(|c| match c {
            'A' | 'B' | 'C' => 3,
            'D' | 'E' | 'F' => 4,
            'G' | 'H' | 'I' => 5,
            'J' | 'K' | 'L' => 6,
            'M' | 'N' | 'O' => 7,
            'P' | 'Q' | 'R' | 'S' => 8,
            'T' | 'U' | 'V' => 9,
            'W' | 'X' | 'Y' | 'Z' => 10,
            _ => 1,
        })
        .sum::<u8>();
    
    println!("{}", res);
}

fn croatian_alphabet() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("stdin error");

    let mut letter_cnt = 0;
    let mut former_c = char::default();
    let mut former_former_c = char::default();
    for c in line.trim().chars() {
        letter_cnt += 1;
        match (former_former_c, former_c, c) {
            (_, 'c', '=') => letter_cnt -= 1,
            (_, 'c', '-') => letter_cnt -= 1,
            ('d', 'z', '=') => letter_cnt -= 2,
            (_, 'd', '-') => letter_cnt -= 1,
            (_, 'l', 'j') => letter_cnt -= 1,
            (_, 'n', 'j') => letter_cnt -= 1,
            (_, 's', '=') => letter_cnt -= 1,
            (_, 'z', '=') => letter_cnt -= 1,
            (_, _, _) => (),
        }
        former_former_c = former_c;
        former_c = c;
    }

    println!("{}", letter_cnt);
}

fn group_word_checker() {
    let [n] = parse_line!(1, usize);
    let mut group_cnt = 0;

    #[inline]
    fn is_group_word(line: String) -> bool {
        let mut alphabets = [false; 26];
        let mut res = true;
        let mut former_c = char::default();

        for c in line.trim().chars() {
            let idx = c as usize - 97; // 'a' => 97
            if former_c != c && alphabets[idx] {
                res = false
            }
            alphabets[idx] = true;
            former_c = c;
        };
        res
    }

    for _ in 0..n {
        // logic
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("stdin error");
        if is_group_word(line) {
            group_cnt += 1;
        };
    };
    println!("{}", group_cnt)
}


fn star_11() {
    let unit = String::from("  *  
 * * 
*****");
    #[inline]
    fn batch(triangle: String) -> String {
        for line in triangle.split("\n") {
            
        }

        String::default()
    }


    #[inline]
    fn inner(num: i32) -> String {
        if num == 1 {
            return UNIT
        } else {
            return batch(inner(num-1))
        }
    }
    let [n] = parse_line!(1, f64);
    let real_n = (n / 3.0).floor().log2() as i32;
}

fn main() {
    star_11();

}
