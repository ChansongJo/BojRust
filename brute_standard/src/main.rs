use std::{convert::{TryFrom}};
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


fn weird_blackjack(m: i32, vec: Vec<i32>) -> i32 {

    let mut diff = i32::MAX;
    let mut answer = 0;
    let mut idx = 0;

    while idx < vec.len() - 2 {
        for i in (idx+1)..(vec.len()-1) {
            for j in (i+1)..vec.len() {
                println!("{} {} {}", vec[idx], vec[i], vec[j]);
                let sum = vec[idx] + vec[i] + vec[j];
                if m - sum >= 0 && m - sum < diff {
                    println!("HIT: {} {}", sum, m -sum);
                    answer = sum;
                    diff = m - sum;
                    if diff == 0 {
                        return answer;
                    }
                }
            }
        }
        idx += 1
    }

    answer
}

fn divide_sum(n: i32) -> i32 {
    // only for even number...
    // 100 a 10 b c  a b c ==> 2c has to be even

    fn _proc(n: i32) -> i32 {
        let mut sum = 0;
        let mut k = n.clone();
        while k > 0 {
            sum += k % 10;
            k /= 10;
        }
        sum + n
    }
    // 143 -> 8 + 143 = 151;
    let mut k = n.clone();
    let mut num_digit = 1;
    while k > 0 {
        k /= 10;
        num_digit += 1;
    }

    // 자릿수...
    for i in (n - (9 * num_digit))..n {
        if _proc(i) == n {
            return i
        }
    }
    0
}

struct Body {
    weight: i32,
    height: i32,
}



fn dungchi(peoples: Vec<Body>) -> Vec::<i32> {
    let mut scores = Vec::<i32>::with_capacity(peoples.len());
    for (i, person) in peoples.iter().enumerate() {
        let mut score = peoples.len() as i32;
        for (j, rival) in peoples.iter().enumerate() {
            if i == j {
                continue;
            }

            if person.weight < rival.weight && person.height < rival.height {
                println!(">> {}", i);
                score -= 1;
            }
        }
        scores.push(peoples.len() as i32 - score + 1);
    }
    
    // scores to order
    println!("{:?}", scores);
    scores
}



const CHESS_BOARD_A: &'static str = "WBWBWBWB
BWBWBWBW
WBWBWBWB
BWBWBWBW
WBWBWBWB
BWBWBWBW
WBWBWBWB
BWBWBWBW";

fn chess_paint(board: Vec<String>, h: i32, w: i32) -> usize {
    let mut res = usize::MAX;

    for i in 0..(h - 8 + 1) {
        for j in 0..(w - 8 + 1) {
            let char_iter = board.iter()
                .skip(i as usize).take(8)
                .flat_map(|x| x.chars().skip(j as usize).take(8));

            let chunk_f = CHESS_BOARD_A.split_whitespace().collect::<String>();
            let forward_diff = chunk_f.chars().zip(char_iter)
                .filter(|(a, b)| a != b)
                .count();
            let backward_diff = 64 - forward_diff;
            res = res.min(forward_diff.min(backward_diff));
        }
    }
    res
}

fn main() {
    let [n] = parse_line!(1, i32);
    
    let answer = doomsday_num(n);
    println!("{}", answer);
}

fn doomsday_num(n: i32) -> i32 {
    let mut cnt = 0;
    'outer: for i in 666..100_000_666 {
        let mut k = i.clone();
        let mut buf = 0;
        while k > 0 {
            if k % 10 == 6 {
                buf += 1;
            } else {
                buf = 0;
            }
            if buf == 3 {
                cnt += 1;
                if cnt == n {
                    return i;
                }
                continue 'outer;
            }
            k /= 10
        }
    }
    0
}