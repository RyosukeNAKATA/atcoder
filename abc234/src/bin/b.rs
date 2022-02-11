use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut mat: [[f64; 2]; n],
    }
    let mut ans = Vec::new();
    fn get_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
    }
    // nC2
    for perm in (0..n).combinations(2) {
        let p1 = perm[0];
        let p2 = perm[1];
        ans.push(get_dist(mat[p1][0], mat[p1][1], mat[p2][0], mat[p2][1]));
    }
    let mut tmp = ans.iter().fold(0.0/0.0, |m, v| v.max(m));
    let tmp = tmp.sqrt();
    println!("{}", tmp);
}