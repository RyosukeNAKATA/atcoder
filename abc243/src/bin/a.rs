use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut v: i32,
        a: i32,
        b: i32,
        c: i32,
    };
    loop {
        v -= a;
        if v < 0 {
            println!("F");
            return;
        }
        v -= b;
        if v < 0 {
            println!("M");
            return;
        }
        v -= c;
        if v < 0 {
            println!("T");
            return;
        }
    }
}
