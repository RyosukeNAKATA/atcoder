use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s_i: u32,
        s_j: u32,
        t_i: u32,
        t_j: u32,
        p: f32,
        h: [Bytes; 20],
        v: [Bytes; 19],
    }

    let mut x = s_i;
    let mut y = s_j;
    let mut answer = String::new();

    for _ in 0..((t_j - s_j) + (t_i - s_i)) / 2 {
        answer += "RD";
    }

    println!("{}", answer);
}
