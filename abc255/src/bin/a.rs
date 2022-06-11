use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        r: usize,
        c: usize,
    }
    let mut a: Vec<Vec<usize>> = vec![];
    for _ in 0..2 {
        input! {
            _a: [usize; 2],
        }
        a.push(_a);
    }
    println!("{}", a[r - 1][c - 1])
}
