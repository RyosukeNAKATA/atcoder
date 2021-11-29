use proconio::input;

fn main() {
    input! {
        n: usize,
        mat: [[i32; 3]; n],
    }

    let mut flag = true;
    let mut t = 0;
    let mut x = 0;
    let mut y = 0;
    for i in 0..n {
        let dt = (mat[i][0] - t).abs();
        let dx = (mat[i][1] - x).abs();
        let dy = (mat[i][2] - y).abs();

        if dx + dy > dt || (dt - (dx + dy)) % 2 != 0 {
            flag = false;
            break;
        }
        t = mat[i][0];
        x = mat[i][1];
        y = mat[i][2];
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
