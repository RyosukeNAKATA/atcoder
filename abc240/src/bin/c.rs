use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize, usize); n],
    };
    let mut dp = vec![false; 10_001];
    dp[0] = true;
    for &(a, b) in &ab {
        for i in (0..=x).rev() {
            if dp[i] {
                dp[i + a] = true;
                dp[i + b] = true;
                dp[i] = false;
            }
        }
    }
    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
