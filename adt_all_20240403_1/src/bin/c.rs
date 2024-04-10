use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    };

    let mut ans = Vec::new();
    let mut buf = Vec::new();
    for i in 1..=n {
        if a.contains(&i) {
            buf.push(i);
        } else {
            buf.push(i);
            buf.reverse();
            ans.extend(buf);
            buf = Vec::new();
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
