use std::str::from_utf8;

use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: u8
    }
    println!("{}", from_utf8(&[n]).unwrap());
}
