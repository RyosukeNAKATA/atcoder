use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;

#[fastout]
fn main() {
    input! {
        k: usize,
    }
    fn base_10_to_2(k: u64) -> String {
        let mut binary = String::new();
        let mut i = k;
        while i > 0 {
            binary.push_str(&format!("{}", i % 2));
            i /= 2;
        }
        binary.chars().rev().collect()
    }
    // Decimal number to Binary number
    let ans = base_10_to_2(k as u64);
    println!("{}", ans.replace("1", "2"));
}