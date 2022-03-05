use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        x: f64,
    };

    if 1 <= x as i32 && x as i32 <= a as i32 {
        println!("1.000000000000");
    } else if b < x {
        println!("0.000000000000");
    } else {
        let ans = c / (b - a);
        println!("{:e}", ans);
    }
}
