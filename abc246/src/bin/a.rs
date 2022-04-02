use std::collections::HashMap;

use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        xy: [i32; 6]
    }
    let mut x_hashmap = HashMap::new();
    let mut y_hashmap = HashMap::new();
    for i in 0..6 {
        if i % 2 == 0 {
            let x_counter = x_hashmap.entry(xy[i]).or_insert(0);
            *x_counter += 1
        } else {
            let y_counter = y_hashmap.entry(xy[i]).or_insert(0);
            *y_counter += 1
        }
    }
    let x = x_hashmap.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let y = y_hashmap.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    println!("{} {}", x.0, y.0);
}
