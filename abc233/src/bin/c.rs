use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i64,
    }
    let mut l = Vec::new();
    let mut ball = Vec::new();
    for _ in 0..n {
        input! {
            a: i64,
        }
        l.push(a);
        input! {
            b: [i64; a],
        }
        b.sort();
        b.dedup();
        ball.push(b);
    }

    let mut ans = 0;
    println!("{:?}", ball);
}