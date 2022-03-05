use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i32,
    };
    let check: i32 = 998244353;
    let mut ans = 0;
    println!("{}", ans / check);
}
