use num_traits::Pow;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    };

    let mut ans = "Takahashi";

    if (a..=b).all(|a| (c..=d).any(|c| is_prime(a + c))) {
        ans = "Aoki"
    }

    println!("{}", ans);
}

fn is_prime(a: u32) -> bool {
    let mut counter = 0;
    for b in 1..=a {
        if a % b == 0 {
            counter += 1;
        }
    }
    counter == 2
}
