use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }

    let mut counter = 0;
    for aa in 0..a+1 {
        for bb in 0..b+1 {
            for cc in 0..c+1 {
                if 500 * aa + 100 * bb + 50 * cc == x {
                    counter += 1;
                }
            }
        }
    }
    println!("{}", counter);
}