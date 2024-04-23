use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: usize,
        ab: [(Usize1,Usize1); q],
    };

    let mut v = vec![0; n];
    for (i, human) in p.iter().enumerate() {
        v[*human] = i;
    }

    for (a, b) in ab {
        if v[a] < v[b] {
            println!("{}", a + 1);
        } else {
            println!("{}", b + 1);
        }
    }
}
