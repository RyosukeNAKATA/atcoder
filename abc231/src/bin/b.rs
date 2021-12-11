use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n:usize,
        mut s: [String; n],
    }

    let mut map = HashMap::new();
    let length = s.len();

    for i in 0..length {
        let count = map.entry(&s[i]).or_insert(0);
        *count += 1;
    }
    let tmp = &map.iter().max_by_key(|v| v.1);
    println!("{}", tmp.unwrap().0);
}
