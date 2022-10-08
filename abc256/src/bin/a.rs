use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: u32,
    }
    println!("{}", 2_i64.pow(n))
}
