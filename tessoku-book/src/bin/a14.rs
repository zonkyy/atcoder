use indexing::algorithms::binary_search;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
        c: [i64; n],
        d: [i64; n],
    };

    let mut ab = Vec::new();
    let mut cd = Vec::new();
    for i in 0..n {
        for j in 0..n {
            ab.push(a[i] + b[j]);
            cd.push(c[i] + d[j]);
        }
    }

    ab.sort();
    cd.sort();
    // cd.reverse();

    let mut i_cd = 0;
    for i_ab in 0..ab.len() {
        if cd.binary_search(&(k - ab[i_ab])).is_ok() {
            println!("Yes");
            return;
        }
        // while i_cd < cd.len() - 1 && ab[i_ab] + cd[i_cd] > k {
        //     i_cd += 1;
        // }

        // if ab[i_ab] + cd[i_cd] == k {
        //     println!("Yes");
        //     return;
        // }
    }

    println!("No");
}
