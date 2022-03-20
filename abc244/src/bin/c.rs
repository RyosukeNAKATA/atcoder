use std::collections::HashSet;
use std::io;
use std::io::Write;
use std::process::exit;
fn main() {
    let n: usize = read();

    let mut set = (1..=((2 * n) + 1)).collect::<HashSet<_>>();
    let mut now: usize = 1_usize;

    loop {
        let now = set.iter().next().unwrap().to_owned();
        println!("{}", set.take(&now).unwrap());
        std::io::stdout().flush().ok();
        let a = read();
        match a {
            0 => exit(0),
            an => set.take(&an),
        };
    }
}

fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().unwrap()
}
