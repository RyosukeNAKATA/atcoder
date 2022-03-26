use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let a_hash: HashSet<i32> = a.into_iter().collect();

    for i in 0..n + 1 {
        if !(a_hash.contains(&(i as i32))) {
            return println!("{}", i);
        }
    }
}
