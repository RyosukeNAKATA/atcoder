use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        abc: Bytes
    }
    let a = abc[0] as i32 - 48;
    let b = abc[1] as i32 - 48;
    let c = abc[2] as i32 - 48;
    let abc = 100 * a + 10 * b + c;
    let bca = 100 * b + 10 * c + a;
    let cba = 100 * c + 10 * a + b;
    println!("{}", abc + bca + cba);
}
