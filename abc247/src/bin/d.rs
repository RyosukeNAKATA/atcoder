use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::io;
#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut queries = vec![];

    for _ in 0..q {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        println!("{:?}", s);
        let tmp: Vec<i64> = s
            .trim()
            .split_whitespace()
            .map(|e| e.parse().ok().unwrap())
            .collect();
        queries.push(tmp);
    }
    println!("{:?}", queries);
    //for x in queries {
    //    println!("{:?}", x);
    //}
}
