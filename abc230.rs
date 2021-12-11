// A
// use proconio::input;

// fn main() {
//     input! {
//     mut n:i32
//     }
//     if n >=42 {
//         n += 1;
//         println!("AGC0{}", n);
//     } else if 10 <= n && n < 42 {
//         println!("AGC0{}", n);
//     } else {
//         println!("AGC00{}", n);
//     }
// }

// B
use proconio::input;

fn main() {
    input! {
    s: proconio::marker::Chars,
    }

    let length = s.len();
    let mut flag = true;

    if length == 2 && s[0 as usize] == 'o' && s[1 as usize] == 'o' {
        println!("No");
        return ();
    } else if length <= 2 {
        println!("Yes");
        return ();
    }

    for i in 0..length - 2 {
        if s[i as usize] == 'o' && s[(i + 1) as usize] == 'x' && s[(i + 2) as usize] == 'x' {
            continue;
        } else if s[i as usize] == 'x' && s[(i + 1) as usize] == 'o' && s[(i + 2) as usize] == 'x' {
            continue;
        } else if s[i as usize] == 'x' && s[(i + 1) as usize] == 'x' && s[(i + 2) as usize] == 'o' {
            continue;
        } else {
            flag = false;
            break;
        }
    }
    
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}

// D
// use proconio::input;

// fn main() {
//     input! {
//         n: usize,
//         d: i64,
//         mat: [[i32; 2]; n],
//     }

//     let mut avalable = 0;
//     let mut t = 0;
//     let mut x = 0;
//     let mut y = 0;
//     for i in 0..n {
//         let dt = (mat[i][0] - t).abs();
//         let dx = (mat[i][1] - x).abs();
//         let dy = (mat[i][2] - y).abs();

//         if dx + dy > dt || (dt - (dx + dy)) % 2 != 0 {
//             flag = false;
//             break;
//         }
//         t = mat[i][0];
//         x = mat[i][1];
//         y = mat[i][2];
//     }

//     if flag {
//         println!("Yes");
//     } else {
//         println!("No");
//     }
// }

// E
// use proconio::input;

// fn main() {
//     input! {
//         n: i64,
//     }

//     let mut ans = 0;
//     for i in 0..n{
//         ans += n / (i+1);
//     }
//     println!("{}", ans)l;
// }
