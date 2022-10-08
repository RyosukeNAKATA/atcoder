use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout,input};
use std::collections::HashSet;
use itertools::Itertools;
#[fastout]
fn main() {
	input!{
        n: usize,
        m: usize,
    }
    let mut k: Vec<usize> = vec![];
    let mut x: Vec<Vec<i32>> = vec![];
    for _ in 0..m {
        input!{
            _k: usize,
            _x: [i32; _k],
        }
        k.push(_k);
        x.push(_x);
    }
    let mut set = HashSet::new();
    for i in 0..m {
        for perm in x[i].iter().permutations(2) {
            set.insert(perm);
        }
    }
    let ans = n * (n-1) / 2;
    if ans == set.len() / 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
