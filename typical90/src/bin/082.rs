use proconio::input;

// a^n (mod m)
fn modpow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut res = 1;
    while n > 0 {
        if n % 2 != 0 {
            res = res * a % m;
        }
        a = a * a % m;
        n /= 2;
    }

    return res;
}

fn modinv(a: usize, p: usize) -> usize {
    return modpow(a, p - 2, p);
}

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let m = 10_usize.pow(9) + 7;

    let l_digit = l.to_string().len() as u32;
    let r_digit = r.to_string().len() as u32;

    let mut ans = 0;
    for digit in l_digit..=r_digit {
        let vl: usize = (l - 1).max(10_usize.pow(digit - 1) - 1) % m;
        let vr: usize = r.min(10_usize.pow(digit) - 1) % m;

        let one_to_vl: usize = (vl + 1) % m * vl % m * modinv(2, m) % m;
        let mut one_to_vr: usize = (vr + 1) % m * vr % m * modinv(2, m) % m;

        if one_to_vr < one_to_vl {
            one_to_vr += m;
        }
        ans += (one_to_vr - one_to_vl) % m * digit as usize % m;
        ans %= m;
    }

    println!("{}", ans);
}
