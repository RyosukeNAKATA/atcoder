use proconio::*;
#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        mut a: [i32; n],
        x: [i32; q],
    }
    a.sort();
    for xi in x {
        println!("{}", n - a.binary_search(&xi).unwrap_or_else(|i| i));
    }
}