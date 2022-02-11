use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }
    let mut counter = 0;
    for i in 0..n {
        if s[i] == t[counter] {
            counter += 1;
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
