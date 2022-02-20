use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32
    };

    let sub = a - b;
    if sub == 1 || sub == -1 || sub == 9 || (a == 10 && b == 1) || (a == 1 && b == 10) {
        println!("Yes");
    } else {
        println!("No");
    }
}
