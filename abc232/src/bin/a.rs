use proconio::*;
use proconio::marker::Bytes;

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    println!("{}", (s[0] as i32 -48)*(s[2] as i32 - 48));
}