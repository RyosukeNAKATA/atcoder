use std::collections::BTreeMap;

use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut s = BTreeMap::new();

    for _ in 0..q {
        input! {
            num: usize,
        }
        if num == 1 {
            input! {
                x1: usize,
            }
            let counter = s.entry(x1).or_insert_with(|| 0);
            *counter += 1;
        } else if num == 2 {
            input! {
                x2: usize,
                c: usize,
            }
            let counter = s.entry(x2).or_default();
            *counter -= c.min(counter.clone());
            if *counter == 0 {
                s.remove(&x2);
            }
        } else {
            let max = *s.iter().last().unwrap().0;
            let min = *s.iter().next().unwrap().0;
            println!("{}", max - min);
        }
    }
}
