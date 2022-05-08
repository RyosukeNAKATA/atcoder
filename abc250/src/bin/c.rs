use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; q],
    }
    let mut answer = (0..n).collect_vec();
    let mut index = (0..n).collect_vec();
    for qx in x {
        let i = index[qx - 1];
        let j = if i < n - 1 { i + 1 } else { i - 1 };
        answer.swap(i, j);
        index.swap(answer[i], answer[j]);
    }
    println!("{}", answer.iter().map(|&x| x + 1).join(" "));
}
