use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let x = 46;

    let mut mod_a: Vec<usize> = vec![0; x];
    for i in a {
        mod_a[i % x] += 1;
    }
    let mut mod_b: Vec<usize> = vec![0; x];
    for i in b {
        mod_b[i % x] += 1;
    }
    let mut mod_c: Vec<usize> = vec![0; x];
    for i in c {
        mod_c[i % x] += 1;
    }

    let mut cnt = 0;
    for ai in 0..x {
        for bi in 0..x {
            let ci = (2 * x - ai - bi) % x;
            cnt += mod_a[ai] * mod_b[bi] * mod_c[ci];
        }
    }

    println!("{}", cnt);
}
