use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    let mut yukichi = -1;
    let mut ichiyoh = -1;
    let mut hideyo = -1;
    for a in 0..=n {
        for b in 0..=n - a {
            let c = n - a - b;
            let total = a * 10_000 + b * 5_000 + c * 1_000;
            if total == y {
                yukichi = a;
                ichiyoh = b;
                hideyo = c;
                println!("{} {} {}", yukichi, ichiyoh, hideyo);
                return ();
            }
        }
    }
    println!("{} {} {}", yukichi, ichiyoh, hideyo);
}
