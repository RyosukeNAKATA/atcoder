use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
    }
    let cos = calc_cos(a, b);
    let sin = calc_sin(b, cos);
    println!("{} {}", cos, sin);
}

fn calc_cos(a: f64, b: f64) -> f64 {
    if a == 0_f64 {
        0_f64
    } else if a > 0_f64 {
        (1_f64 / (1_f64 + (b.powf(2_f64) / a.powf(2_f64)))).sqrt()
    } else {
        -((1_f64 / (1_f64 + (b.powf(2_f64) / a.powf(2_f64)))).sqrt())
    }
}

fn calc_sin(b: f64, cos: f64) -> f64 {
    if b == 0_f64 {
        0_f64
    } else if b > 0_f64 {
        (1_f64 - cos.powf(2_f64)).sqrt()
    } else {
        -((1_f64 - cos.powf(2_f64)).sqrt())
    }
}
