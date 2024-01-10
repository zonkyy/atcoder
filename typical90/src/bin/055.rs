use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        mut a: [usize; n],
    }

    let mut cnt = 0;
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        if a[i] * a[j] % p * a[k] % p * a[l] % p * a[m] % p == q {
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", cnt);
}
