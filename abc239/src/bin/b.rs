use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        x: i64
    };
    let mut ans = x / 10;
    if x < 0 && x % 10 != 0 {
        ans -= 1
    }
    println!("{}", ans);
}
