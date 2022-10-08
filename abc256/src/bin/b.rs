use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }
    let mut ans_q = 0;
    let mut map = vec![false; 4];
    for i in 0..a.len() {
        let ai = a[i];
        map[0] = true;
        for j in 0..4 {
            if map[3 - j] == true && 3 < 3 - j + ai as usize {
                ans_q += 1;
                map[3 - j] = false;
            } else if map[3 - j] == true {
                map[3 - j] = false;
                map[3 - j + ai as usize] = true;
            }
        }
    }
    println!("{}", ans_q);
}
