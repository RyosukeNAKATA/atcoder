use itertools::Itertools;
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [i32; n],
    }
    let mut stand = 0;
    for i in 0..n {
        if stand >= h[i] {
            break;
        }
        stand = h[i];
    }
    println!("{}", stand);
}
