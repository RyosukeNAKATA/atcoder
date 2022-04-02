use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: i64,
        k: i64,
        x: i64,
        mut a: [i64; n],
    }
    a.sort();
    a.reverse();

    let mut tmp = k;
    for i in 0..n {
        let y = tmp.min(a[i as usize] / x);
        tmp -= y;
        a[i as usize] -= x * y;
    }

    a.sort();
    a.reverse();
    for i in 0..tmp.min(n) {
        a[i as usize] = 0;
    }

    let answer = a.iter().sum::<i64>();
    println!("{}", answer);
}
