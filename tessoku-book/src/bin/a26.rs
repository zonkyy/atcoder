use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        q: usize,
        x: [usize; q],
    };
    const LIMIT: usize = 300000;

    let mut sieve = vec![true; LIMIT + 1];
    let mut idx = 2;
    while idx < 550 {
        if !sieve[idx] {
            idx += 1;
            continue;
        }

        for i in (idx + 1)..=LIMIT {
            if i % idx == 0 {
                sieve[i] = false;
            }
        }

        idx += 1;
    }

    for &i in x.iter() {
        if sieve[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
