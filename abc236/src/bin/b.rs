use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; 4 * n - 1],
    }
    let mut hashmap = HashMap::new();
    for res in a {
        let counter = hashmap.entry(res).or_insert(0);
        *counter += 1;
    }
    let min = hashmap.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    println!("{}", min.0);
}