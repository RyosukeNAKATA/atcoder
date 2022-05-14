use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: String
    }
    let n = 6 / s.len();

    for _ in 0..n {
        print!("{}", s);
    }
    println!("");
}
