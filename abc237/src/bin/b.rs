use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input!{
        h: usize,
        w: usize,
        mat_a: [[u32; w]; h],
    }
    let mut ans = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            ans[j][i] = mat_a[i][j];
        }
    }
    for i in 0..w {
        for j in 0..h {
            print!("{} ", ans[i][j]);
        }
        println!();
    }
}