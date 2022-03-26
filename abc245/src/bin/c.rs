use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n],
    }

    let flag = solve(&a, &b, 0, n, k);
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn solve(a: &Vec<i32>, b: &Vec<i32>, depth: usize, n: usize, k: i32) -> bool {
    if depth == n - 1 {
        return true;
    }

    if (a[depth + 1] - a[depth]).abs() <= k {
        solve(a, b, depth + 1, n, k);
    }
    if (b[depth + 1] - a[depth]).abs() <= k {
        solve(a, b, depth + 1, n, k);
    }
    if (b[depth + 1] - b[depth]).abs() <= k {
        solve(a, b, depth + 1, n, k);
    }
    if (a[depth + 1] - b[depth]).abs() <= k {
        solve(a, b, depth + 1, n, k);
    }
    false
}
