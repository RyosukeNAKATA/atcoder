use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [u32; n],
    }
    let mut tmp = p[..k].to_vec();
    tmp.sort();
    tmp.reverse();
    println!("{}", tmp[k-1]);
    for i in k..n {
        tmp.push(p[i]);
        tmp.sort();
        tmp.reverse();
        println!("{}", tmp[k-1]);
    }
}