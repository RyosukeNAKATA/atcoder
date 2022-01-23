use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};
use proconio::*;

#[fastout]
fn main() {
    input!{
        mut s: Chars, 
        a: usize,
        b: usize,
    }
    let s_a = s[a-1];
    let s_b = s[b-1];
    s[a-1] = s_b;
    s[b-1] = s_a;
    println!("{}", s.iter().collect::<String>());
}