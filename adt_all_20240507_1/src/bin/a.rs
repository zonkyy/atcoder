use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: usize,
        f: usize,
        x: usize,
    };

    let mut taka = 0;
    for i in 0..x {
        if i % (a + c) < a {
            taka += b;
        }
    }

    let mut ao = 0;
    for i in 0..x {
        if i % (d + f) < d {
            ao += e;
        }
    }

    if taka > ao {
        println!("Takahashi");
    } else if taka < ao {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
