use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }
    let mut answer = vec![0; n];
    for i in 0..n {
        answer[i] = i + 1;
    }

    let mut hashmap = HashMap::new();
    for i in 0..n {
        let res = answer[i];
        let counter = hashmap.entry(res).or_insert(0);
        *counter = i;
    }
    //println!("{:?}", hashmap);

    for i in 0..q {
        let xq = x[i];
        let mut index = hashmap.get(&xq).unwrap();
        if *index == n - 1 {
            let tmp = *index;
        }
    }

    //for i in 0..q {
    //    let xq = x[i];
    //    let index = answer.iter().position(|a| a == &xq).unwrap();
    //    if index == n - 1 {
    //        let tmp = answer[index];
    //        answer[index] = answer[index - 1];
    //        answer[index - 1] = tmp;
    //    } else {
    //        let tmp = answer[index];
    //        answer[index] = answer[index + 1];
    //        answer[index + 1] = tmp;
    //    }
    //}

    for i in 0..n {
        if i == n - 1 {
            println!("{}", answer[i]);
        } else {
            print!("{} ", answer[i]);
        }
    }
}
