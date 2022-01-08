// A
use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input! {
        t: i32
    }
    fn f(x: i32)->i32{
        x*x + 2 * x + 3
    }
    println!("{}",f(f(f(t)+t)+f(f(t))));
}

// B
// use proconio::marker::{Bytes, Chars};
// use proconio::{fastout,input};
// use itertools::Itertools;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         mut mat: [[f64; 2]; n],
//     }
//     let mut ans = Vec::new();
//     fn get_dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
//         (x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)
//     }
//     // nC2
//     for perm in (0..n).combinations(2) {
//         let p1 = perm[0];
//         let p2 = perm[1];
//         ans.push(get_dist(mat[p1][0], mat[p1][1], mat[p2][0], mat[p2][1]));
//     }
//     let mut tmp = ans.iter().fold(0.0/0.0, |m, v| v.max(m));
//     let tmp = tmp.sqrt();
//     println!("{}", tmp);
// }

// C
// use proconio::marker::{Bytes, Chars, Usize1};
// use proconio::*;

// #[fastout]
// fn main() {
//     input! {
//         k: usize,
//     }
//     fn base_10_to_2(k: u64) -> String {
//         let mut binary = String::new();
//         let mut i = k;
//         while i > 0 {
//             binary.push_str(&format!("{}", i % 2));
//             i /= 2;
//         }
//         binary.chars().rev().collect()
//     }
//     // Decimal number to Binary number
//     let ans = base_10_to_2(k as u64);
//     println!("{}", ans.replace("1", "2"));
// }

// D
// use proconio::marker::{Bytes, Chars, Usize1};
// use proconio::*;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         k: usize,
//         p: [u32; n],
//     }
//     let mut tmp = p[..k].to_vec();
//     tmp.sort();
//     tmp.reverse();
//     println!("{}", tmp[k-1]);
//     for i in k..n {
//         tmp.push(p[i]);
//         tmp.sort();
//         tmp.reverse();
//         println!("{}", tmp[k-1]);
//     }
// }