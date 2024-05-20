use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    const UPPER: usize = 10usize.pow(6) + 9;

    let mut sorted_a = a.clone();
    sorted_a.sort();
    // i よりも大きな要素の和
    let mut acc = vec![0; UPPER];
    let mut acctmp = sorted_a.iter().sum();
    acc[0] = acctmp;
    let mut ai = 0;
    for i in 1..UPPER {
        while ai < n && sorted_a[ai] == i {
            acctmp -= i;
            ai += 1;
        }
        acc[i] = acctmp;
    }

    let mut ans = Vec::new();
    for &i in a.iter() {
        ans.push(acc[i].to_string());
    }

    println!("{}", ans.join(" "));
}
