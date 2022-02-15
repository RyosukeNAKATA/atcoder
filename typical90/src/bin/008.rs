use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    //input! {
    //    n: usize,
    //    s: Chars,
    //}
    //let num_check = 10_i64.pow(9) + 7;
    //let atcoder = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];

    //let mut dp = vec![vec![0; atcoder.len() + 1]; n + 1];
    //for i in 0..n + 1 {
    //    dp[i][0] = 1
    //}

    //for i in 0..n {
    //    for j in 0..atcoder.len() {
    //        if s[i] == atcoder[j] {
    //            dp[i + 1][j + 1] = dp[i][j] + dp[i][j + 1];
    //            dp[i + 1][j + 1] %= num_check;
    //        } else {
    //            dp[i + 1][j + 1] = dp[i][j + 1];
    //            dp[i + 1][j + 1] %= num_check;
    //        }
    //    }
    //}
    //println!("{}", dp[n][atcoder.len()]);

    input! {
        _n: usize,
        s: String,
    }
    let atcoder = "atcoder";
    let num_check = 10_i64.pow(9) + 7;

    let mut dp = vec![0; atcoder.len() + 1];
    dp[0] = 1;
    for c in s.chars() {
        if let Some(x) = atcoder.find(c) {
            dp[x + 1] = (dp[x] + dp[x + 1]) % num_check;
        }
    }
    println!("{}", dp[7]);
}
