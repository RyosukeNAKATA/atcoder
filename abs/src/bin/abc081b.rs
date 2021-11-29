use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    let mut counter = 0;
    'outer: loop {
        for i in 0..n {
            if a[i] % 2 == 1 {
                break 'outer;
            } else {
                a[i] /= 2;
            }
        }
        counter += 1;
    };
    println!("{}", counter);
}