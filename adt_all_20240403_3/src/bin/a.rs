use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    // Vec<char> を空白区切りで join する
    println!(
        "{}",
        s.iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
