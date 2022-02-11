use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input!{
        n: u32,
    }
    if n >= 5 {
        println!("Yes");
    } else if 2_u32.pow(n as u32) > n.pow(2 as u32) {
        println!("Yes");
    } else {
        println!("No");
    }
}
