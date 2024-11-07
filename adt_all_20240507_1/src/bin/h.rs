use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: [usize; 9],
    };

    let mut min = 1000000000;
    let mut min_i = 0;
    for (idx, &i) in c.iter().enumerate() {
        if i <= min {
            min = i;
            min_i = idx;
        }
    }

    let mut ans_v = vec![0; 9];
    let mut cnt = n / min;
    let mut rest = n % min;
    for i in ((min_i + 1)..=8).rev() {
        while rest >= c[i] - min {
            ans_v[i] += 1;
            rest -= (c[i] - min);
            cnt -= 1;
        }
    }

    ans_v[min_i] = cnt;

    for i in (0..9).rev() {
        for _ in 0..ans_v[i] {
            print!("{}", i + 1);
        }
    }
    println!();
}
