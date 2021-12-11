// A
// use proconio::input;

// fn main() {
//     input! {
//     mut n:f32
//     }
//     println!("{}", n/100 as f32);
// }

// B
// use std::collections::HashMap;
// use proconio::input;

// fn main() {
//     input! {
//         n:usize,
//         mut s: [String; n],
//     }

//     let mut map = HashMap::new();
//     let length = s.len();

//     for i in 0..length {
//         let count = map.entry(&s[i]).or_insert(0);
//         *count += 1;
//     }
//     let tmp = &map.iter().max_by_key(|v| v.1);
//     println!("{}",tmp.unwrap().0);
// }

// C
// use proconio::input;
// fn main() {
//     input! {
//         n:usize,
//         q:usize,
//         mut a: [i32; n],
//         mut x: [i32; q],
//     }
//     for i in 0..q {
//         println!("{}",a.iter().enumerate().filter(|&(_, &v)| v >= x[i]).count());
//         }
// }
use proconio::input;
fn main() {
    input! {
        n:usize,
        q:usize,
        mut a: [i32; n],
        mut x: [i32; q],
    }
    let mut ans = Vec::new();

    for i in 0..q {
        let mut tmp = 0;
        for j in &a{
            if x[i] <= *j {
                tmp += 1;
            }
        }
        ans.push(tmp);
    }

    for k in 0..q {
        println!("{}", ans[k as usize]);
    }
}