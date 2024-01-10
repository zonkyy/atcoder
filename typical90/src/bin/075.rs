use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    // 素因数分解
    let mut cnt = 0;
    let mut m = 2;
    let sqrt = n.sqrt();
    while n > 1 {
        if n % m == 0 {
            n /= m;
            cnt += 1;
            continue;
        }

        m += 1;

        // sqrt(n) 以上の素数
        if m > sqrt {
            cnt += 1;
            break;
        }
    }

    // 答え算出
    let ans = match cnt {
        1 => 0,
        _ => (cnt as f64 - 1.0).log2().floor() as i32 + 1,
    };
    println!("{}", ans);
}
