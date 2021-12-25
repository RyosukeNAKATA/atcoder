use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x: i32,
        y: i32,
    }
    let mut counter = 0;
    loop {
        if x >= y {
            println!("{}", counter);
            break;
        }
        x += 10;
        counter += 1;
    }
}
