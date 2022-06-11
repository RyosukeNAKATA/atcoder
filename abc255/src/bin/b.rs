use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; k],
    }
    let mut xy: Vec<(i64, i64)> = vec![];
    let mut light: Vec<(i64, i64)> = vec![];
    for i in 0..n {
        input! {
            _x: i64,
            _y: i64,
        }
        if a.iter().any(|e| *e == i + 1) {
            light.push((_x, _y));
        } else {
            xy.push((_x, _y));
        }
    }

    let mut candidates: Vec<f64> = vec![];
    for z in xy {
        let mut candidate_dir = vec![];
        for l in &light {
            let tmp = calc(*l, z);
            candidate_dir.push(tmp);
        }
        let min = candidate_dir.into_iter().fold(0.0 / 0.0, f64::min);
        candidates.push(min);
    }
    let min_r = candidates.into_iter().fold(0.0 / 0.0, f64::max);
    println!("{}", min_r);
}

fn calc(xy1: (i64, i64), xy2: (i64, i64)) -> f64 {
    (((xy1.0 - xy2.0).abs().pow(2) + (xy1.1 - xy2.1).abs().pow(2)) as f64).sqrt()
}
