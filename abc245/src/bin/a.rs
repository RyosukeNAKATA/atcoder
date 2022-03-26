use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    if a < c {
        return println!("Takahashi");
    } else if a > c {
        return println!("Aoki");
    } else {
        if b <= d {
            return println!("Takahashi");
        } else {
            return println!("Aoki");
        }
    }
}
