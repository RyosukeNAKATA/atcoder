use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut vecdeq = VecDeque::new();

    for _ in 0..q {
        input! {z: usize}
        if z == 1 {
            input! {
                x: usize,
                c: usize,
            }
            vecdeq.push_back((x, c));
        } else {
            input! {
                mut c: usize,
            }

            let mut sum = 0;
            while c > 0 {
                let (x, k) = vecdeq.pop_front().unwrap();
                if k < c {
                    sum += x * k;
                    c -= k;
                } else {
                    vecdeq.push_front((x, k - c));
                    sum += x * c;
                    c = 0;
                }
            }
            println!("{}", sum);
        }
    }
}
