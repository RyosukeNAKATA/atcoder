use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut mat = vec![];
    for _ in 0..h {
        input! {
            _s: Chars,
        }
        mat.push(_s);
    }

    let mut position: Vec<_> = vec![];

    for i in 0..h {
        for j in 0..w {
            if mat[i][j] == 'o' {
                position.push((i as i32, j as i32));
            }
        }
    }

    let mut length = (position[0].0 - position[1].0).abs() + (position[0].1 - position[1].1).abs();

    println!("{}", length);
}
