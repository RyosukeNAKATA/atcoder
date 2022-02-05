use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u128,
    }
    let mut answer = 0;
    let mut number_of_digits = 1;
    while number_of_digits <= n {
        let n_max = u128::min(number_of_digits * 10 - 1, n);
        let n_under = number_of_digits - 1;
        let tmp = n_max - n_under;
        answer += sum_1_to_n(tmp);
        number_of_digits *= 10;
    }
    println!("{}", answer % 998244353);
}

fn sum_1_to_n(n: u128) -> u128 {
    n * (n + 1) / 2
}
