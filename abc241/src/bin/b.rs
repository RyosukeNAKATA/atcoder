use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };
    let mut a_hashmap = HashMap::new();
    for x in a {
        let counter = a_hashmap.entry(x).or_insert(0);
        *counter += 1;
    }
    let mut b_hashmap = HashMap::new();
    for y in b {
        let counter = b_hashmap.entry(y).or_insert(0);
        *counter += 1;
    }

    for (key, value) in &b_hashmap {
        //let a_i = a_hashmap.get(&key).unwrap();

        match a_hashmap.get(&key) {
            Some(a_i) => {
                if value > a_i {
                    println!("No");
                    return;
                }
            }
            None => return println!("No"),
        }
    }
    println!("Yes");
}
