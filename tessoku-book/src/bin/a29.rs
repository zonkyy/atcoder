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
    while i < 1000000007 {
        let last = powers.last().unwrap();
        let val = (last * last) % MOD;
        powers.push(val);
        i *= 2;
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
