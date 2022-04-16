use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut s: Bytes
    }

    s.sort();

    for i in 0..9 {
        if s[i] - 48 != i as u8 {
            return println!("{}", i);
        }
    }
    println!("{}", 9);
}
