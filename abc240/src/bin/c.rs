use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        x: u64,
    };
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        input! {
            _a: u64,
            _b: u64,
        };
        a.push(_a);
        b.push(_b);
    }
    for bit in 0..(1 << n) {
        let mut tmp_sum = 0_u64;
        for i in 0..n {
            if (bit >> i & 1) == 1 {
                tmp_sum += a[i];
            } else {
                tmp_sum += b[i];
            }
            if tmp_sum > x {
                continue;
            }
        }
        if tmp_sum == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
