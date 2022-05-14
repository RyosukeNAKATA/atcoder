use im_rc::HashSet;
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut unique = HashSet::new();
    let mut answer: (usize, usize) = (0, 0);

    for i in 0..n {
        input! {
            s: String,
            t: usize,
        }
        if !unique.contains(&s) {
            unique.insert(s);
            if answer.0 < t {
                answer.0 = t;
                answer.1 = i;
            } else {
                continue;
            }
        }
    }
    println!("{}", answer.1 + 1);
}
