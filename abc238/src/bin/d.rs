use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input!{
        n: u64,
    }
    let n_str = &n.to_string();
    let number_of_digits = n_str.len() as u32;
    let mut ans = 0;
    for i in 0..number_of_digits - 1 {
        ans += (sum_of_digits(10_u64.pow(i as u32 + 1) - 1) - sum_of_digits(10_u64.pow(i as u32) - 1));
        println!("ans1: {}", ans);
    }
    ans += (sum_of_digits(n) - sum_of_digits(10_u64.pow(number_of_digits - 1) - 1));
    println!("final: {}", (sum_of_digits(n - (10_u64.pow(number_of_digits - 1) - 1)) - sum_of_digits(10_u64.pow(number_of_digits - 1) - 1)));
    println!("ans2: {}", ans);
    println!("{}", ans%998244353);
}

fn sum_of_digits(n: u64) -> u64 {
    n * (n + 1) / 2
}