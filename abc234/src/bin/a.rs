use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input! {
        t: i32
    }
    fn f(x: i32)->i32{
        x*x + 2 * x + 3
    }
    println!("{}",f(f(f(t)+t)+f(f(t))));
}