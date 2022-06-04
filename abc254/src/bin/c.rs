use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [i32; n],
    }
    let mut model = a.clone();
    model.sort();

    for i in 0..n - k {
        if a[i] > a[i + k] {
            a.swap(i, i + k);
        }
        if model == a {
            return println!("Yes");
        }
    }
    println!("No");
}
