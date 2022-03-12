use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut x: Vec<i32> = vec![];
    let mut y: Vec<i32> = vec![];
    for _ in 0..n {
        input! {
            _x: i32,
            _y: i32,
        }
        x.push(_x);
        y.push(_y);
    }
    input! {
        s: Chars,
    }

    let max_x = x.iter().max().unwrap();
    let max_y = y.iter().max().unwrap();

    let mut mat = vec![vec![0; *max_x as usize + 1]; *max_y as usize + 1];

    for i in 0..n {
        if s[i] == 'R' {
            mat[y[i] as usize][x[i] as usize] = 1;
        } else {
            mat[y[i] as usize][x[i] as usize] = -1;
        }
    }

    for v in mat {
        let mut flag = false;
        for num in v {
            if flag == true && num == 1 {
                return println!("Yes");
            } else if num == -1 {
                flag = true
            }
        }
    }
    println!("No");
}
