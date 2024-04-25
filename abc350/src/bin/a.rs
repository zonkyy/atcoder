use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    if s[0..3].iter().collect::<String>() != "ABC".to_string() {
        println!("No");
        return;
    }

    let n = s[3..6].iter().collect::<String>();
    if n >= "001".to_string() && n <= "349".to_string() && n != "316".to_string() {
        println!("Yes");
        return;
    }

    println!("No");
}
