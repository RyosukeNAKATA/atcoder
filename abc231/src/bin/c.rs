use proconio::input;
fn main() {
    input! {
        n:usize,
        q:usize,
        mut a: [i32; n],
        mut x: [i32; q],
    }
    for i in 0..q {
        println!("{}",a.iter().enumerate().filter(|&(_, &v)| v >= x[i]).count());
        }
}

// use proconio::input;
// fn main() {
//     input! {
//         n:usize,
//         q:usize,
//         mut a: [i32; n],
//         mut x: [i32; q],
//     }
//     let mut ans = Vec::new();

//     for i in 0..q {
//         let mut tmp = 0;
//         for j in &a{
//             if x[i] <= *j {
//                 tmp += 1;
//             }
//         }
//         ans.push(tmp);
//     }

//     for k in 0..q {
//         println!("{}", ans[k as usize]);
//     }
// }