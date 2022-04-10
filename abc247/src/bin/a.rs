use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars
    }
    println!("0{}{}{}", s[0], s[1], s[2]);
}
