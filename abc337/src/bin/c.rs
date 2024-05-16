use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let mut first = 0;
    let mut memo = vec![0; n];
    for i in 0..n {
        if a[i] == -1 {
            first = i;
        } else {
            memo[a[i] as usize - 1] = i;
        }
    }

    let mut ans = vec![(first + 1).to_string()];
    let mut i = first;
    for _ in 0..n - 1 {
        i = memo[i];
        ans.push((i + 1).to_string());
    }

    println!("{}", ans.join(" "));
}
