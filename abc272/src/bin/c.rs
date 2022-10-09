use itertools::Itertools;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    a.sort();
    a.reverse();
    let mut odd_nums = vec![];
    let mut even_nums = vec![];
    for i in 0..n {
        if a[i] % 2 == 0 {
            even_nums.push(a[i]);
        } else {
            odd_nums.push(a[i]);
        }
    }
    odd_nums.sort();
    odd_nums.reverse();
    even_nums.sort();
    even_nums.reverse();

    let mut ans = -1;
    if 2 <= odd_nums.len() {
        ans = ans.max(odd_nums[0] + odd_nums[1]);
    }
    if 2 <= even_nums.len() {
        ans = ans.max(even_nums[0] + even_nums[1]);
    }
    println!("{}", ans);
}
