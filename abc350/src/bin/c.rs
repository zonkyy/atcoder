use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [Usize1; n],
    };

    let mut place = vec![0; n];
    for i in 0..n {
        place[a[i]] = i;
    }

    let mut ans = Vec::new();
    for i in 0..n {
        if a[i] != i {
            let j = place[i];
            a.swap(i, j);
            place.swap(i, a[j]);
            ans.push((i, j));
        }
    }

    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i + 1, j + 1);
    }
}
