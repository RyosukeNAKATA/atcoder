// A
use proconio::marker::{Bytes, Chars};
use proconio::{fastout,input};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    println!("{}", (s[0] as i32 -48)*(s[2] as i32 - 48));
}

// B
// use proconio::marker::{Bytes, Chars};
// use proconio::{fastout,input};

// #[fastout]
// fn main() {
//     input! {
//         mut s: Bytes,
//         t: Bytes,
//     }

//     let mut flag = false;
//     let length = s.len();

//     for i in 0..26 {
//         let mut tmp = Vec::new();
//         let mut min_flag = 0;
//         for j in 0..length {
//             if s[j] + i as u8 >= 123 {
//                 tmp.push(s[j] + i as u8 - 26);
//             } else {
//                 tmp.push(s[j] + i as u8);
//             }
//             if tmp[j] == t[j] {
//                 min_flag += 1;
//             } else {
//                 continue;
//             }
//         }
//         if min_flag == length {
//             flag = true;
//             break;
//         }
//     }

//     if flag {
//         println!("Yes");
//     } else {
//         println!("No");
//     }
// }

// C
// use proconio::marker::{Bytes, Chars};
// use proconio::{fastout,input};

// #[fastout]
// fn main() {
//     input! {
//         n:usize,
//         m:usize,
//         mut mat: [[usize; 2]; m * 2],
//     }
//     let mut a = Vec::new();
//     let mut b = Vec::new();
//     let mut c = Vec::new();
//     let mut d = Vec::new();
//     for i in 0..m{
//         a.push(mat[i][0]);
//         b.push(mat[i][1]);
//     }
//     for i in 0..m{
//         c.push(mat[i + m][0]);
//         d.push(mat[i + m][1]);
//     }
//     let mut flag = false;



//     if flag {
//         println!("Yes");
//     } else {
//         println!("No");
//     }
// }