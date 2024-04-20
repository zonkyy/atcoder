use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    };

    let mut acc = vec![0];
    for i in 0..n {
        acc.push(acc[i] + a[i]);
    }

    for (l, r) in lr {
        let ok = acc[r] - acc[l - 1];
        let ng = (r - l + 1) - ok;
        if ok > ng {
            println!("win");
        } else if ok < ng {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}
