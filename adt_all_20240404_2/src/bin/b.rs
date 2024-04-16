use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    let mut ans = Vec::new();
    for c in s {
        if !['a', 'i', 'u', 'e', 'o'].contains(&c) {
            ans.push(c);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
