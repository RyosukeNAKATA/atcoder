use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut ans: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        let mut tmp: Vec<usize> = vec![];
        for j in 0..i + 1 {
            if j == 0 || j == i {
                tmp.push(1);
            } else {
                tmp.push(ans[i - 1][j - 1] + ans[i - 1][j]);
            }
        }
        println!(
            "{}",
            tmp.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        ans.push(tmp);
    }
}
