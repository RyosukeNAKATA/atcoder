// A
use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input! {
        abc: Bytes
    }
    let a = abc[0] as i32 -48;
    let b = abc[1] as i32 -48;
    let c = abc[2] as i32 -48;
    let abc = 100 * a + 10 * b + c;
    let bca = 100 * b + 10 * c + a;
    let cba = 100 * c + 10 * a + b;
    println!("{}",abc+bca+cba);
}

// B
// use proconio::marker::{Bytes, Chars};
// use proconio::{fastout,input};
// use itertools::Itertools;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         mut h: [i32; n],
//     }
//     let mut stand = 0;
//     for i in 0..n {
//         if stand >= h[i] {
//             break;
//         }
//         stand = h[i];
//     }
//     println!("{}", stand);
// }

// C
// use proconio::marker::{Bytes, Chars, Usize1};
// use proconio::*;
// use std::collections::HashMap;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         q: usize,
//         a: [i32; n],
//         mut queries: [[i32; 2]; q],
//     }
//     let mut ans = vec![-1; q];
//     let mut hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
//     for i in 0..a.len() {
//         if hashmap.contains_key(&a[i]) {
//             hashmap.get_mut(&a[i]).unwrap().insert(0, i as i32+1);
//         } else {
//             hashmap.insert(a[i], vec![i as i32 +1]);
//         }
//     }

//     for i in 0..q {
//         if !hashmap.contains_key(&queries[i][0]) {
//             break;
//         } else {
//             let x = queries[i][0];
//             let k = queries[i][1];
//             let tmp = hashmap.get(&x).unwrap().len() as i32;
//             if k > tmp {
//                 ans[i] = -1;
//             } else {
//                 ans[i] = hashmap.get(&x).unwrap()[tmp as usize - k as usize];
//             }
//         }
//     }

//     for i in &ans[..] {
//         println!("{}", *i);
//     }
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