use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {mut s: Chars};
    s.sort();
    for i in &s {
        print!("{}", i);
    }
    println!("");
}
