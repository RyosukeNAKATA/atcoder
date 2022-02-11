use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    let mut ans = Vec::new();
    for i in 1..=n {
        let mut ii = i;
        let mut tmp = 0;
        while ii > 0 {
            tmp += ii % 10;
            ii /= 10;
        }
        if a <= tmp && tmp <= b {
            ans.push(i);
        }
    }
    let mut sum = 0;
    for j in &ans {
        sum += j;
    }
    println!("{}", sum);
}
