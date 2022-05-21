use std::collections::HashMap;
use std::collections::HashSet;

use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Bytes; n],
    }
    let mut slot: Vec<Vec<u8>> = vec![];
    for ai in a {
        slot.push(ai.iter().map(|x| x - 48).collect());
    }

    let mut num_list = vec![0; n];

    let mut hashmap = HashMap::new();
    for s in slot {
        for (i, x) in s.into_iter().enumerate() {
            let counter = hashmap.entry(x).or_insert(HashSet::new());
            counter.insert(i);
        }
    }

    //println!("{:?}", hashmap);
    //for i in hashmap {
    //    println!("{:?}", i);
    //}

    let mut vec = vec![];
    for h in hashmap {
        vec.push(h)
    }

    vec.sort_by(|a, b| (-(a.1.len() as i32)).cmp(&(-(b.1.len() as i32))));

    //println!("{:?}", vec);
    //println!("{:?}", vec[0]);
    //println!("{:?}", vec[0].1);
    println!("{:?}", type_of(&vec[0].1));
    //println!("{:?}", vec[0].1.len());
}
fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}
