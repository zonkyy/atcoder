use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
        lrv: [(usize, usize, i64); q],
    }

    let mut inconv: Vec<i64> = Vec::new();
    for w in a.windows(2) {
        inconv.push(w[1] - w[0]);
    }

    let mut sum: i64 = inconv.iter().map(|i| i.abs()).sum();
    for (l, r, v) in lrv {
        if l >= 2 {
            let new_v = inconv[l - 2] + v;
            sum += new_v.abs() - inconv[l - 2].abs();
            inconv[l - 2] = new_v;
        }
        if r <= n - 1 {
            let new_v = inconv[r - 1] - v;
            sum += new_v.abs() - inconv[r - 1].abs();
            inconv[r - 1] = new_v;
        }

        println!("{}", sum);
    }
}
