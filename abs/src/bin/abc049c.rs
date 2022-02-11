use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let length = s.len();
    let mut position = length;

    while position > 0 {
        if position >= 7 && s[position - 7..position] == "dreamer".to_string() {
            position -= 7;
        } else if position >= 6 && s[position - 6..position] == "eraser".to_string() {
            position -= 6;
        } else if position >= 5 && s[position - 5..position] == "dream".to_string() {
            position -= 5;
        } else if position >= 5 && s[position - 5..position] == "erase".to_string() {
            position -= 5;
        } else {
            break;
        }
    }

    if position == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
