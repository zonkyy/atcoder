use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        a: usize,
        mut b: usize,
    };
    const MOD: usize = 1000000007;

    let mut powers = vec![a];
    let mut i = 2;
    let mut cnt = 0;
    while cnt < 62 {
        let last = powers.last().unwrap();
        let val = (last * last) % MOD;
        powers.push(val);
        i *= 2;
        cnt += 1;
    }
    i /= 2;

    let mut ans = 1;
    while b > 0 {
        let val = powers.pop().unwrap();
        if b >= i {
            b -= i;
            ans *= val;
            ans %= MOD;
        }

        i /= 2;
    }

    println!("{}", ans);
}
