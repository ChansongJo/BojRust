// https://www.acmicpc.net/source/20352949
// IMPL:parse_line
macro_rules! parse_line { ($($t: ty),+) => ({
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    ($(iter.next().unwrap().parse::<$t>().unwrap()),+)
  })}
  
  // returns largest 'k' where 'B[k] = v' for given 'v'
  // ==> calculates how many entries are there smaller than v in A[][]
  fn f(v :u32) -> u32 {
    (1..=v).map(|x| v/x).sum()
  }
  
  fn f_for_fast1(v :u32) -> u32 {
    let sqrtv = (v as f64).sqrt() as u32;
  
    let (mut sum, mut x) = (0, 1);
    while x < sqrtv {
      sum += v / x;
      x += 1;
    }
  
    let mut i = v / x;
    while i > 0 {
      sum += i * (v/i - v/(i+1));
      i -= 1;
    }
  
    return sum;
  }
  
  fn f_for_fast2(v :u32) -> u32 {
    let sqrtv = (v as f64).sqrt() as u32;
  
    let (mut sum, mut x) = (0, 1);
    while x < sqrtv {
      sum += v / x;
      x += 1;
    }
  
    let mut i = v / x;
    let overlap = (x-1) * i;
    while i > 0 {
      sum += v/i;
      i -= 1;
    }
  
    return sum - overlap;
  }
  
  use std::cmp::min;
  
  fn f_crop_by(n :u32, v :u32) -> u32 {
    (1..=min(n,v)).map(|x| min(n,v/x)).sum()
  }
  
  fn f_for_fast2_crop_by(n :u32, v :u32) -> u32 {
    let sqrtv = (v as f64).sqrt() as u32;
  
    let (mut sum, mut x) = (0, 1);
    while x < sqrtv {
      sum += min(n, v/x);
      x += 1;
    }
  
    let mut i = min(n, v/x);
    let overlap = (x-1) * i;
    while i > 0 {
      sum += min(n, v/i);
      i -= 1;
    }
  
    return sum - overlap;
  }
  
  fn main() {
    /* it works perfect!
    for v in 1 .. 1000 {
      let (ret_naive, ret_opt) = (f(v), f_for_fast1(v));
      if ret_naive != ret_opt {
        println!("f({}) = {} != {} = ff({})", v, ret_naive, ret_opt, v);
      }
    }
    for v in 1 .. 1000 {
      let (ret_naive, ret_opt) = (f(v), f_for_fast2(v));
      if ret_naive != ret_opt {
        println!("f({}) = {} != {} = ff({})", v, ret_naive, ret_opt, v);
      }
    }
    for n in 1 .. 100 {
      for v in 1 ..= n*n {
        let (ret_naive, ret_opt) = (f_crop_by(n,v), f_for_fast2_crop_by(n,v));
        if ret_naive != ret_opt {
          println!("f({},{}) = {} != {} = ff({},{})", n, v, ret_naive, ret_opt, n, v);
        }
      }
    } */
  
    /*
                             k
    .. v-1 | v-1 | v-1 | v | v | v | v+1 | v+1 | ..
                   ^f(v-1)       ^f(v)
    find v where
      f(v-1) < k <= f(v)
    apply B[] on each term
      B[f(v-1)] < B[k] <= B[f(v)]
    f is inverse of B so that
      v-1 < B[k] <= v
    ... B[k] = v
  
    f is monotonically increasing.
    [s,e] indicates possible positions of 'v'
    if f(m) < k then f(..m) < k => s := m
    if k < f(m) then k < f(m..) => e := m
    if k = f(m) then k <= f(m..) => e := m
    */
  
    let n = parse_line!(u32);
    let k = parse_line!(u32);
  
    let (mut s, mut e) = (0, n*n); // f(0) < k <= f(n*n) verified
    while s+1 < e {
      let m = (s + e) / 2;
      if k <= f_for_fast2_crop_by(n, m) {
        e = m;
      } else {
        s = m;
      }
    }
  
    println!("{}", e);
  }