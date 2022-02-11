use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut a = Vec::new();
    for i in 0..(2*n - 1) {
        input! {
            n_i: [u32; 2*n - i - 1],
        }
        a.push(n_i);
    }

    for combi in (0..2*n).combinations(2) {
        
    }
}