use proconio::input;

fn main() {
    input! {
    mut n:i32
    }
    if n >=42 {
        n += 1;
        println!("AGC0{}", n);
    } else if 10 <= n && n < 42 {
        println!("AGC0{}", n);
    } else {
        println!("AGC00{}", n);
    }
}