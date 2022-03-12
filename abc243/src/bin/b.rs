use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    };
    let mut ans_1: Vec<i32> = vec![];
    let mut ans_2: Vec<i32> = vec![];
    for i in 0..n {
        if a[i] == b[i] {
            ans_1.push(a[i])
        } else if a.iter().any(|e| e == &b[i]) {
            ans_2.push(b[i])
        }
    }
    println!("{}", ans_1.len());
    println!("{}", ans_2.len());
}
