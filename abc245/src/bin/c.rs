use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    }

    let mut ok_a = true;
    let mut ok_b = true;
    for i in 0..n - 1 {
        let old_ok_a = ok_a;
        let old_ok_b = ok_b;
        ok_a = (old_ok_a && (a[i + 1] - a[i]).abs() <= k)
            || (old_ok_b && (a[i + 1] - b[i]).abs() <= k);
        ok_b = (old_ok_a && (b[i + 1] - a[i]).abs() <= k)
            || (old_ok_b && (b[i + 1] - b[i]).abs() <= k);
    }

    let flag = ok_a || ok_b;
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
