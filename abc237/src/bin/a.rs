use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input!{
        n: i64,
    }
    if - (2_i64.pow(31 as u32)) <= n && n < (2_i64.pow(31 as u32)) {
        println!("Yes");
    } else {
        println!("No");
    }
}