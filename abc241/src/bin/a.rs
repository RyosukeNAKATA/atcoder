use std::usize;

use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: [usize; 10],
    };
    let mut now = 0;
    for i in 0..3 {
        now = a[now];
    }
    println!("{}", now);
}
