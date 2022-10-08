use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout,input};
use itertools::Itertools;
#[fastout]
fn main() {
	input!{
        n: usize,
        mut a: [i32; n],
    }
    if a.len() == 2 && (a[0] + a[1]) % 2 == 1 {
        return println!("-1");
    }
    a.sort();
    a.reverse();
    let mut ans = vec![];
    for comb in a.iter().combinations(2) {
        let b = comb[0] + comb[1];
        ans.push(b);
    }
    ans.sort();
    ans.reverse();
    for i in ans.iter() {
        if i % 2 == 0 {
            return println!("{}", i);
        }
    }
}
