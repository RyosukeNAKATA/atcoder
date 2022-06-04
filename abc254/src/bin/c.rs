use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i32; n],
    }
    let mut ans = vec![0; n];
    for i in 0..k {
        let mut tmp = vec![];
        for j in (i..n).step_by(k) {
            tmp.push((a[j], j));
        }
        tmp.sort();
        let mut j = i;
        for &(t, _) in tmp.iter() {
            ans[j] = t;
            j += k;
        }
    }
    a.sort();
    if ans == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
