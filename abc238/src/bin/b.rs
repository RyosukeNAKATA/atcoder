use std::usize;

use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut ans = Vec::new();
    let mut now = 360;
    let mut tmp: Vec<i32> = Vec::new();
    tmp.push(0);
    tmp.push(now);
    for i in a {
        if now - i >= 0 {
            now -= i;
            tmp.push(now);
        } else {
            now += 360 - i;
            tmp.push(now);
        }
    }
    tmp.sort();
    for i in 0..tmp.len() - 1 {
        ans.push(tmp[i + 1] - tmp[i]);
    }
    println!("{}", ans.iter().max().unwrap());
}
