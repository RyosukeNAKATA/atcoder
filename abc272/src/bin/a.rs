use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout,input};
#[fastout]
fn main() {
	input!{
        n: usize,
        a: [i32; n],
    }
    println!("{}", a.iter().sum::<i32>());
}
