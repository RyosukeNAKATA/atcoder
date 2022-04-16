use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let num_check = 998244353;

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..n + 1 {
        dp[i][0] = 1
    }
    println!("{}", dp[n][n]);
}
