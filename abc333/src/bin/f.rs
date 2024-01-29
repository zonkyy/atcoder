use proconio::input;

fn modinv(a: i64, m: i64) -> i64 {
    let mut a = a;
    let mut b = m;
    let mut u = 1;
    let mut v = 0;

    while b != 0 {
        let t = a / b;
        a -= t * b;
        a ^= b;
        b ^= a;
        a ^= b;
        u -= t * v;
        u ^= v;
        v ^= u;
        u ^= v;
    }

    u %= m;
    if u < 0 {
        u += m;
    }

    u
}

fn main() {
    input! {
        n: usize,
    };

    println!("{}", 2 * modinv(3, 998244353));
}
