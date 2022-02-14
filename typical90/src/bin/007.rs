use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
        b: [i32; q]
    };
    let inf = 1_000_000_000 + 1;
    a.push(-inf);
    a.push(2 * inf);
    a.sort();
    for b in b {
        let i = a.binary_search_by_key(&(b, 0), |x| (*x, 1)).unwrap_err();
        let ans = (a[i] - b).min(b - a[i - 1]);
        println!("{}", ans);
    }
}
