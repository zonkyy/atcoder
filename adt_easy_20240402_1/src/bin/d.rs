use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
        f: u64,
    };

    const MOD: u64 = 998244353;
    let mut left: u64 = ((a % MOD) * (b % MOD)) % MOD;
    left = (left * (c % MOD)) % MOD;
    let mut right: u64 = ((d % MOD) * (e % MOD)) % MOD;
    right = (right * (f % MOD)) % MOD;

    if left >= right {
        println!("{}", left - right);
    } else {
        println!("{}", MOD + left - right);
    }
}
