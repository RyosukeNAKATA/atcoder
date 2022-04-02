use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut k: i32,
        x: i32,
        mut a: [i32; n],
    }
    a.sort();
    a.reverse();

    for i in 0..a.len() {
        if a[i] < x {
            continue;
        } else {
            let tmp = k;
            'inside: for _ in 0..tmp {
                if k == 0 || a[i] < x {
                    break 'inside;
                } else {
                    a[i] += -x;
                    k += -1;
                }
            }
        }
    }

    let mut i = 0;
    while k > 0 {
        a[i] -= x;
        if a[i] < 0 {
            a[i] = 0;
        }
        k += -1;
        i += 1;
        if i == a.len() {
            break;
        }
    }

    let answer: i32 = a.iter().sum();
    println!("{}", answer);
}
