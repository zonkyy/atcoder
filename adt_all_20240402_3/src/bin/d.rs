use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    };

    let mut min = 10000000;
    for si in 0..=n {
        for mi in 0..=n {
            for li in 0..=n {
                let cnt = 6 * si + 8 * mi + 12 * li;
                let price = s * si + m * mi + l * li;
                if cnt >= n && price < min {
                    min = price;
                }
            }
        }
    }

    println!("{}", min);
}
