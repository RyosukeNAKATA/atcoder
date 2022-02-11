use proconio::marker::{Bytes, Chars, Usize1};
use proconio::*;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        mut queries: [[i32; 2]; q],
    }
    let mut ans = vec![-1; q];
    let mut hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..a.len() {
        if hashmap.contains_key(&a[i]) {
            hashmap.get_mut(&a[i]).unwrap().insert(0, i as i32 + 1);
        } else {
            hashmap.insert(a[i], vec![i as i32 + 1]);
        }
    }

    for i in 0..q {
        if !hashmap.contains_key(&queries[i][0]) {
            break;
        } else {
            let x = queries[i][0];
            let k = queries[i][1];
            let tmp = hashmap.get(&x).unwrap().len() as i32;
            if k > tmp {
                ans[i] = -1;
            } else {
                ans[i] = hashmap.get(&x).unwrap()[tmp as usize - k as usize];
            }
        }
    }

    for i in &ans[..] {
        println!("{}", *i);
    }
}
