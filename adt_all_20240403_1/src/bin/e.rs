use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n * 3],
    };

    let mut ans = Vec::new();
    let mut memo = vec![0; n];
    for i in a {
        memo[i - 1] += 1;
        if memo[i - 1] == 2 {
            ans.push(i.to_string());
        }
    }

    println!("{}", ans.join(" "));
}
