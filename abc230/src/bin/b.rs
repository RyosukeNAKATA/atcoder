use proconio::input;

fn main() {
    input! {
    s: proconio::marker::Chars,
    }

    let length = s.len();
    let mut flag = true;

    if length == 2 && s[0 as usize] == 'o' && s[1 as usize] == 'o' {
        println!("No");
        return ();
    } else if length <= 2 {
        println!("Yes");
        return ();
    }

    for i in 0..length - 2 {
        if s[i as usize] == 'o' && s[(i + 1) as usize] == 'x' && s[(i + 2) as usize] == 'x' || s[i as usize] == 'x' && s[(i + 1) as usize] == 'o' && s[(i + 2) as usize] == 'x' || s[i as usize] == 'x' && s[(i + 1) as usize] == 'x' && s[(i + 2) as usize] == 'o' {
            continue;
        } else {
            flag = false;
            break;
        }
    }

    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}