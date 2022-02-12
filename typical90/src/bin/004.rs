use std::usize;

use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mat_a: [[u32; w]; h],
    }

    let mut row = vec![0; h];
    let mut column = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            row[i] += mat_a[i][j];
            column[j] += mat_a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", row[i] + column[j] - mat_a[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        println!("");
    }
}
