use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut ans = Vec::new();
    for i in 0..(n - 1) {
        ans.push(a[i]);
        let diff = if a[i] < a[i + 1] { 1 } else { -1 };
        let mut tmp = a[i];
        while (tmp - a[i + 1]).abs() > 1 {
            tmp += diff;
            ans.push(tmp);
        }
    }
    ans.push(a[n - 1]);

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
