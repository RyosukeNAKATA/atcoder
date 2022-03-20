use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::io;
use std::io::Write;
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut num_list = vec![];
    for i in 0..2 * n + 1 {
        num_list.push((i + 1) as i32)
    }

    for _ in 0..n + 1 {
        println!("{}", num_list[0]);
        io::stdout().flush().unwrap();
        input! {
            a: i32
        }
        if a == 0 {
            return;
        } else {
            num_list.remove(num_list.binary_search(&a).unwrap());
        }
    }
}
