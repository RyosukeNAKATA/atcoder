use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut ans = String::new();

    let mut start = 0;
    for end in (n - k)..n {
        let u = &s[start..=end];
        let (i, &c) = u.iter().enumerate().min_by_key(|(_, &x)| x).unwrap();

        start += i + 1;
        ans.push(c);
    }
    println!("{}", ans);
}
