// IMPL:parse_line
use std::{convert::{TryFrom, TryInto}, ops::RangeToInclusive};

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

fn main() {
    // hansu is on the (x, y) pos
    // rectangle's left under corner is (0, 0)
    // rectangle's right upper corner is (w, h)
    // get the minium length to the side of the rec.
    let [r] = parse_line!(1, u32);
    let euc_circle = PI * r.pow(2) as f64;
    let taxi_circle = taxi_geometry(r);
    println!("{}\n{}", euc_circle, taxi_circle)
}

use std::f64::consts::PI;
fn taxi_geometry(r: u32) -> f64 {
    /*
    19세기 독일 수학자 헤르만 민코프스키는 비유클리드 기하학 중 택시 기하학을 고안했다.

    택시 기하학에서 두 점 T1(x1,y1), T2(x2,y2) 사이의 거리는 다음과 같이 구할 수 있다.

    D(T1,T2) = |x1-x2| + |y1-y2|

    두 점 사이의 거리를 제외한 나머지 정의는 유클리드 기하학에서의 정의와 같다.

    따라서 택시 기하학에서 원의 정의는 유클리드 기하학에서 원의 정의와 같다.

    원: 평면 상의 어떤 점에서 거리가 일정한 점들의 집합

    반지름 R이 주어졌을 때, 유클리드 기하학에서 원의 넓이와, 택시 기하학에서 원의 넓이를 구하는 프로그램을 작성하시오. */

    let taxi_circle = (2 * r).pow(2) as f64 / 2.0;
    taxi_circle
}
