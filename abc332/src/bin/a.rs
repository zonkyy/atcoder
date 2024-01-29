use proconio::input;

fn main() {
    input! {
        (n, s, k): (usize, usize, usize),
        pq: [(usize, usize); n],
    };

    let mut sum = 0;
    for (p, q) in pq {
        sum += p * q;
    }

    if sum < s {
        sum += k;
    }

    println!("{}", sum);
}
