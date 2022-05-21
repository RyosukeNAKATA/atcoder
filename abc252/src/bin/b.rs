use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
        b: [usize; k],
    }

    let max_deliciousness = a.iter().max().unwrap();
    for bi in b {
        if &a[bi - 1] == max_deliciousness {
            return println!("Yes");
        }
    }
    println!("No");
}
