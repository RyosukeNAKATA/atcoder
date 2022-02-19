use nalgebra::ComplexField;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    };
    let candidates = vec![
        [1, 2],
        [2, 1],
        [-1, 2],
        [-2, 1],
        [1, -2],
        [2, -1],
        [-1, -2],
        [-2, -1],
    ];
    for canditate1 in &candidates {
        for canditate2 in &candidates {
            if x1 + canditate1[0] + canditate2[0] == x2 && y1 + canditate1[1] + canditate2[1] == y2
            {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
