// A
use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input! {
        mut x: i32,
        y: i32,
    }
    if x >= y {
        println!("{}", 0);
    } else {
        let mut counter = 0;
        loop {
            counter += 1;
            x += 10;
            if x >= y {
                println!("{}", counter);
                break;
            }
        }
    }
}

// B
// use proconio::marker::{Bytes, Chars};
// use proconio::{fastout,input};

// #[fastout]
// fn main() {
//     input! {
//         l: usize,
//         r: usize,
//         s: Chars,
//     }

//     let mut start = Vec::new();
//     let mut end = Vec::new();
//     let mut tmp = Vec::new();
//     for i in 0..s.len() {
//         if i < l - 1 {
//             start.push(s[i]);
//         } else if r <= i {
//             end.push(s[i]);
//         }else  {
//             tmp.push(s[i]);
//         } }
//     tmp.reverse();
//     println!("{}{}{}", start.iter().collect::<String>(),tmp.iter().collect::<String>(),end.iter().collect::<String>());
// }

// C
// use proconio::marker::{Bytes, Chars, Usize1};
// use proconio::*;
// use superslice::Ext;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         x: i64,
//     }
//     let mut l = Vec::new();
//     let mut ball = Vec::new();
//     for _ in 0..n {
//         input! {
//             a: i64,
//         }
//         l.push(a);
//         input! {
//             b: [i64; a],
//         }
//         b.sort();
//         b.dedup();
//         ball.push(b);
//     }

//     let mut ans = 0;
//     println!("{:?}", ball);
// }