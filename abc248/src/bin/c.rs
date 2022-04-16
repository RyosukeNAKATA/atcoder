use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    const MOD: u64 = 998244353;

    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=k {
            for ai in 1..=m {
                if j + ai <= k {
                    dp[i + 1][j + ai] += dp[i][j];
                    dp[i + 1][j + ai] %= MOD;
                }
            }
        }
    }
    println!("{}", dp[n].iter().fold(0, |s, c| (s + c) % MOD));
}
