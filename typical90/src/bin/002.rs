use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    };
    let mut ans = vec![];
    for bit in 0..(1 << n) {
        let mut counter = 0;
        let mut flag = true;
        for i in 0..n {
            if (bit >> i & 1) == 1 {
                counter += 1;
            } else {
                counter -= 1;
            }
            if counter < 0 {
                flag = false;
                break;
            }
        }
        let mut str = "".to_string();
        if flag && counter == 0 {
            for i in 0..n {
                if (bit >> i & 1) == 1 {
                    str = str + "(";
                } else {
                    str = str + ")";
                }
            }
            ans.push(str);
        }
    }
    ans.sort();
    for c in ans {
        println!("{}", c);
    }
}
