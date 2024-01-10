use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut c1: Vec<usize> = vec![0];
    let mut c2: Vec<usize> = vec![0];
    let (mut c1_last, mut c2_last): (usize, usize) = (0, 0);
    for (c, p) in cp {
        c1_last = if c == 1 { p + c1_last } else { c1_last };
        c1.push(c1_last);
        c2_last = if c == 2 { p + c2_last } else { c2_last };
        c2.push(c2_last);
    }

    for (l, r) in lr {
        println!("{} {}", c1[r] - c1[l - 1], c2[r] - c2[l - 1]);
    }
}
