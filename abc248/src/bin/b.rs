use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut a: usize,
        b: usize,
        k: usize,
    }

    let mut counter = 0;
    while a < b {
        a = a * k;
        counter += 1;
    }

    println!("{}", counter);
}
