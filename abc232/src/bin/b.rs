use proconio::*;
use proconio::marker::Bytes;

#[fastout]
fn main() {
    input! {
        mut s: Bytes,
        t: Bytes,
    }

    let mut flag = false;
    let length = s.len();

    for i in 0..26 {
        let mut tmp = Vec::new();
        let mut min_flag = 0;
        for j in 0..length {
            if s[j] + i as u8 >= 123 {
                tmp.push(s[j] + i as u8 - 26);
            } else {
                tmp.push(s[j] + i as u8);
            }
            if tmp[j] == t[j] {
                min_flag += 1;
            } else {
                continue;
            }
        }
        if min_flag == length {
            flag = true;
            break;
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}