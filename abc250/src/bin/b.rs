use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    for i in 0..a * n {
        for j in 0..b * n {
            let ia = i / a;
            let jb = j / b;
            if ia % 2 == 0 && jb % 2 == 0 {
                print!(".");
            } else if ia % 2 == 1 && jb % 2 == 1 {
                print!(".");
            } else if ia % 2 == 0 && jb % 2 == 1 {
                print!("#");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}
