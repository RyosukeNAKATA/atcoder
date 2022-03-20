use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        t: Chars,
    }
    let mut x = 0;
    let mut y = 0;
    let mut flag = 0;
    for c in t {
        if c == 'R' {
            flag += 1;
        } else if c == 'S' {
            if flag % 4 == 0 {
                x += 1
            } else if flag % 4 == 1 {
                y += -1
            } else if flag % 4 == 2 {
                x += -1
            } else {
                y += 1
            }
        }
    }
    println!("{} {}", x, y);
}
