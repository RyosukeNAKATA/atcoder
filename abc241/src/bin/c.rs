use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    };
    let mut s = Vec::new();
    for _ in 0..n {
        input! {
            _s: Chars
        }
        s.push(_s)
    }
}
