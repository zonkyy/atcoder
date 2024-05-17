use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut r: [usize; n],
        queries: [usize; q],
    };
    r.sort();

    let mut acc = Vec::new();
    let mut tmp = 0;
    for &i in r.iter() {
        tmp += i;
        acc.push(tmp);
    }

    for &i in queries.iter() {
        let ans = acc.partition_point(|&x| x <= i);
        println!("{}", ans);
    }
}
