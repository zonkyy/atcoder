use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    let mut scores = Vec::new();
    for (a, b) in ab {
        scores.push(b);
        scores.push(a - b);
    }

    scores.sort();
    scores.reverse();
    let ans: usize = scores[0..k].iter().sum();
    println!("{}", ans);
}
