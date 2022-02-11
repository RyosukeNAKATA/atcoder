use proconio::marker::Usize1;
use proconio::*;
use superslice::Ext;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(Usize1, Usize1); m],
        mut cd: [(Usize1, Usize1); m],
    }
    ab.sort();
    cd.sort();
    if ab == cd {
        println!("Yes");
        return ();
    }

    let mut p = (0..n).collect::<Vec<_>>();
    while p.next_permutation() {
        let mut tmp = Vec::new();
        for &(c, d) in &cd {
            tmp.push((p[c].min(p[d]), p[c].max(p[d])));
        }
        tmp.sort();
        if ab == tmp {
            println!("Yes");
            return ();
        }
    }
    println!("No");
}
