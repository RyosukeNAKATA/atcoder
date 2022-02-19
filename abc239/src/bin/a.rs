use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: f64
    };
    println!("{}", (h * (12_800_000 as f64 + h)).sqrt());
}
