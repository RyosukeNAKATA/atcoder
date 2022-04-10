use std::collections::VecDeque;

use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let ans = calc(n);
    for x in 0..ans.len() {
        if x < ans.len() - 1 {
            print!("{} ", ans[x]);
        } else {
            println!("{}", ans[x]);
        }
    }
}

fn calc(n: usize) -> VecDeque<usize> {
    let mut tmp = VecDeque::new();
    if n == 1 {
        tmp.push_front(1);
    } else {
        for i in calc(n - 1) {
            tmp.push_back(i);
        }
        tmp.push_back(n);
        for j in calc(n - 1) {
            tmp.push_back(j);
        }
    }
    tmp
}
