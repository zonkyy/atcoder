use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [Usize1; k],
    };

    let mut max = 0;
    let mut max_indices = Vec::new();
    for (k, v) in a.iter().enumerate() {
        if max == *v {
            max_indices.push(k);
        } else if max < *v {
            max = *v;
            max_indices = vec![k];
        }
    }

    if b.iter().any(|&x| max_indices.contains(&x)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
