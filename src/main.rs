// A
use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};
use proconio::*;

#[fastout]
fn main() {
    input!{
        mut s: Chars, 
        a: usize,
        b: usize,
    }
    let s_a = s[a-1];
    let s_b = s[b-1];
    s[a-1] = s_b;
    s[b-1] = s_a;
    println!("{}", s.iter().collect::<String>());
}

// B
// use proconio::marker::{Bytes, Chars};
// use proconio::{fastout,input};
// use std::collections::HashMap;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         a: [i32; 4 * n - 1],
//     }
//     let mut hashmap = HashMap::new();
//     for res in a {
//         let counter = hashmap.entry(res).or_insert(0);
//         *counter += 1;
//     }
//     let min = hashmap.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
//     println!("{}", min.0);
// }

// C
// use proconio::marker::{Bytes, Chars, Usize1};
// use proconio::*;
// use std::collections::HashMap;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         s: [String; n],
//         t: [String; m],
//     }
//     let mut counter = 0;
//     for i in 0..n {
//         if s[i] == t[counter] {
//             counter += 1;
//             println!("Yes");
//         } else {
//             println!("No");
//         }
//     }
// }

// D
// use itertools::Itertools;
// use proconio::marker::{Bytes, Chars, Usize1};
// use proconio::*;

// #[fastout]
// fn main() {
//     input! {
//         n: usize,
//     }
//     let mut a = Vec::new();
//     for i in 0..(2*n - 1) {
//         input! {
//             n_i: [u32; 2*n - i - 1],
//         }
//         a.push(n_i);
//     }

//     for combi in (0..2*n).combinations(2) {
        
//     }
// }

// bit all choice
// use proconio::marker::{Bytes, Chars};
// use proconio::{fastout,input};
// use proconio::*;

// #[fastout]
// fn main() {
//     input!{n:usize, k:usize}
//     input! {a:[usize;n]}

//     let first = a[0];
//     let mut min_ans = 10u128.pow(18);
    
//     for bit in 0..(1 << n) {
//         let mut sum = 0;
//         let mut cost = 0;
//         let mut cnt = 0;
//         let mut  beforeH = first;
//         for i in 1..n { //a[0]はせんたくしないので1..から始める
//             if (bit >> i & 1)  == 1 {//伸ばすと決めた建物
//                 cnt += 1; //cnt = 4 で５つの建物が見える
//                 if beforeH >= a[i] {
//                     cost = beforeH + 1 - a[i];
//                     sum += cost;
//                     beforeH += 1;
//                 }else{ //すでに伸ばす必要がないとき
//                     //cost is 0
//                     beforeH = a[i];
//                 }
//             }
//             beforeH = beforeH.max(a[i]);
//         }
//         if cnt +1 >= k && min_ans > sum as u128{
//             min_ans = sum as u128;
//         }
//     }
//     println!("{}", min_ans);
// }