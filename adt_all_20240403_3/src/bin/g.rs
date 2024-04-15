use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    };

    let mut v = vec![(0, 0); 200009];
    for (l, r) in lr {
        v[l].0 += 1;
        v[r].1 += 1;
    }

    let mut ans = Vec::new();
    let mut first_l = 0;
    let mut l_rest = 0;
    for i in 0..v.len() {
        if v[i] == (0, 0) {
            continue;
        }

        if l_rest == 0 && v[i].0 > 0 {
            first_l = i;
        }
        l_rest += v[i].0;
        l_rest -= v[i].1;
        if l_rest == 0 && v[i].0 != v[i].1 {
            ans.push((first_l, i));
        }
    }

    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
