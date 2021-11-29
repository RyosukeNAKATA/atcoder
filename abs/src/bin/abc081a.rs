use proconio::input;

fn main() {
    input! {
        a: String,
    }
    let mut counter = 0;
    for c in a.chars() {
        match c {
            '1' => counter += 1,
            _ => (),
        }
    }
    println!("{}", counter);
}
