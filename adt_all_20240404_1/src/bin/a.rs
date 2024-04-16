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

    let mut takahashi = 0;
    let mut aoki = 0;
    for i in 0..x {
        let j = i % (a + c);
        if j < a {
            takahashi += b;
        }

        let k = i % (d + f);
        if k < d {
            aoki += e;
        }
    }

    if takahashi > aoki {
        println!("Takahashi");
    } else if takahashi < aoki {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
