use proconio::input;

// a^b (mod m)
fn binpower(mut a: usize, mut b: usize, m: usize) -> usize {
    let mut ans = 1;
    while b >= 1 {
        if b % 2 == 1 {
            ans = ans * a % m;
        }
        a = a * a % m;
        b /= 2;
    }

    return ans;
}

fn main() {
    input! {
        mut n: usize,
        mut k: usize,
    }
    let m = 10_usize.pow(9) + 7;

    match (n, k) {
        (1, _) => println!("{}", k),
        (_, 1) => println!("0"),
        (2, _) => println!("{}", k * (k - 1)),
        (_, 2) => println!("0"),
        _ => println!("{}", k * (k - 1) % m * binpower(k - 2, n - 2, m) % m),
    }
}
