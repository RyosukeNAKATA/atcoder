use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    if a <= b && b <= c || c <= b && b <= a {
        return println!("Yes");
    } else {
        return println!("No");
    }
}
