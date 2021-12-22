use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        p: i64,
        q: i64,
        r: i64,
        s: i64,
    }

    for i in p..=q {
        let mut ans = String::new();
        for j in r..=s {
            if (i - j == a - b) || (i + j == a + b) {
                ans += "#";
            } else {
                ans += ".";
            }
        }
        println!("{}", ans);
    }
}
