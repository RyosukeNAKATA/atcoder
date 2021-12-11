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
    let mut ok = 0;
    let mut ng = l + 1;
    while 1 < (ng - ok).abs() {
        let mid = (ok + ng) / 2;
        if solve(l, k, &a, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

fn solve(l: i32, k: i32, a: &Vec<i32>, m: i32) -> bool {
    let mut cnt = 0;
    let mut pre = 0;
    for i in a {
        if i - pre >= m && m <= l - i {
            pre = *i;
            cnt += 1;
        }
    }
    k <= cnt
}
