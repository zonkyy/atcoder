use std::collections::HashSet;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    const N: usize = 9;
    input! {
        a: [[usize; N]; N],
    };

    for i in 0..N {
        let mut row = HashSet::new();
        for j in 0..N {
            row.insert(a[i][j]);
        }
        let mut col = HashSet::new();
        for j in 0..N {
            col.insert(a[j][i]);
        }

        let rule = row.len() == 9
            && col.len() == 9
            && row.iter().sum::<usize>() == 45
            && row.iter().sum::<usize>() == 45;
        if !rule {
            println!("No");
            return;
        }
    }

    for r in [0, 3, 6] {
        for c in [0, 3, 6] {
            let mut square = HashSet::new();
            for i in [0, 1, 2] {
                for j in [0, 1, 2] {
                    square.insert(a[r + i][c + j]);
                }
            }

            let rule = square.len() == 9 && square.iter().sum::<usize>() == 45;
            if !rule {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
