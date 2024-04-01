use proconio::input;

fn main() {
    input! {
        s: String,
    };

    let mut s = s.chars().collect::<Vec<char>>();
    if s[0] != '<' {
        println!("No");
        return;
    }

    if s[s.len() - 1] != '>' {
        println!("No");
        return;
    }

    let m = &s[1..s.len() - 1];
    if m.iter().all(|c| *c == '=') {
        println!("Yes");
        return;
    }

    println!("No");
}
