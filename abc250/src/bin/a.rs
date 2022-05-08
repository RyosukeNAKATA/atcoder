use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: i32,
        w: i32,
        r: i32,
        c: i32,
    }
    let mut counter = 0;

    if 1 <= r - 1 {
        counter += 1;
    }
    if r + 1 <= h {
        counter += 1;
    }
    if 1 <= c - 1 {
        counter += 1;
    }
    if c + 1 <= w {
        counter += 1;
    }

    println!("{}", counter);
}
