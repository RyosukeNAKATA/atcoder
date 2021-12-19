use proconio::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        mut mat: [[usize; 2]; m * 2],
    }
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    let mut d = Vec::new();
    for i in 0..m{
        a.push(mat[i][0]);
        b.push(mat[i][1]);
    }
    for i in 0..m{
        c.push(mat[i + m][0]);
        d.push(mat[i + m][1]);
    }
    let mut flag = false;

    

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}