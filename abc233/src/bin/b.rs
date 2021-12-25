use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input! {
        l: usize,
        r: usize,
        s: Chars,
    }

    let mut start = Vec::new();
    let mut end = Vec::new();
    let mut tmp = Vec::new();
    for i in 0..s.len() {
        if i < l - 1 {
            start.push(s[i]);
        } else if r <= i {
            end.push(s[i]);
        }else  {
            tmp.push(s[i]);
        } }
    tmp.reverse();
    println!("{}{}{}", start.iter().collect::<String>(),tmp.iter().collect::<String>(),end.iter().collect::<String>());
}