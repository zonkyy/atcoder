use ac_library::ModInt1000000007 as Mint;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        r: usize,
    };

    if n == r {
        println!("1");
        return;
    }

    // 分子
    let mut numer = Mint::new(1);
    for i in (r + 1)..=n {
        numer *= i;
    }

    // 分母
    let mut denom = Mint::new(1);
    for i in 1..=(n - r) {
        denom *= i;
    }

    println!("{}", numer * denom.inv());
}

#[fastout]
fn main2() {
    input! {
        n: usize,
        r: usize,
    };
    const MOD: usize = 1000000007;

    if n == r {
        println!("1");
        return;
    }

    // 分子
    let mut numer = 1;
    for i in (r + 1)..=n {
        numer *= i;
        numer %= MOD;
    }

    // 分母
    let mut denom = 1;
    for i in 1..=(n - r) {
        denom *= i;
        denom %= MOD;
    }

    // denom^(MOD-2)
    let mut val = 1;
    let mut mask = 1;
    for _ in 0..62 {
        if (MOD - 2) & mask != 0 {
            val *= denom;
            val %= MOD;
        }
        denom *= denom;
        denom %= MOD;
        mask <<= 1;
    }

    println!("{}", (numer * val) % MOD);
}
