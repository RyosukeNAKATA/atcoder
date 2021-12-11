use proconio::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        l:i32,
        k:i32,
        mut a:[i32; n],
    }
    a.sort();
    let mut left = 0;
    let mut right = l + 1;
    while (right - left).abs() > 1 {
        let mid = (left + right) / 2;
        if solve(l, k, &a, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}

#[fastout]
fn solve(l: i32, k: i32, a: &Vec<i32>, m: i32) -> bool {
    let mut cnt = 0;
    let mut pre = 0;
    for i in a {
        if m <= i - pre && m <= l - i {
            pre = *i;
            cnt += 1;
        }
    }
    k <= cnt
}
