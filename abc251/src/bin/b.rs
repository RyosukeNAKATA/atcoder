use std::collections::HashSet;
use std::iter::FromIterator;

use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        mut a: [i32; n],
    }
    let mut answer = 0;
    let mut avalable = a.clone();

    for comb in a.iter().combinations(2) {
        avalable.push(comb.into_iter().sum::<i32>());
    }
    for comb in a.iter().combinations(3) {
        avalable.push(comb.into_iter().sum::<i32>());
    }

    let set: HashSet<i32> = HashSet::from_iter(avalable.iter().cloned());
    for i in set {
        if i <= w as i32 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
