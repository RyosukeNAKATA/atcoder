use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };
    let mut hashset: HashSet<u32> = a.into_iter().collect();
    println!("{}", hashset.len());
}
