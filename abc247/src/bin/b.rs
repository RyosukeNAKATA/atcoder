use std::collections::HashMap;

use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize
    }
    let mut first_name = vec![];
    let mut family_name = vec![];
    for _ in 0..n {
        input! {
            _s: String,
            _t: String,
        }
        first_name.push(_t);
        family_name.push(_s);
    }
    let mut hash_map = HashMap::new();
    for x in first_name.iter() {
        let counter = hash_map.entry(x).or_insert(0);
        *counter += 1;
    }
    for y in family_name.iter() {
        let counter = hash_map.entry(y).or_insert(0);
        *counter += 1;
    }

    let mut flag = true;

    for i in 0..n {
        if first_name[i] == family_name[i] {
            if hash_map[&first_name[i]] > 2 {
                flag = false;
                break;
            }
        } else if hash_map[&first_name[i]] > 1 && hash_map[&family_name[i]] > 1 {
            flag = false;
            break;
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
