use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input!{
        mut s: Chars,
    }
    let mut rev = s.clone();
    rev.reverse();
    let mut flag = false;
    let n = s.len();
    for _ in 0..n-1 {
        if s == rev {
            flag = true;
            break;
        }
        rev.push('a');
        s = rev.clone();
        s.reverse();
    }
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}