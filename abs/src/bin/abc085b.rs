use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut d: [i32; n],
    }
    let mut uniq = HashSet::new();
    for i in 0..n {
        uniq.insert(d[i]);
    }
    println!("{}", uniq.len());
}
