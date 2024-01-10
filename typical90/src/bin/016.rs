use proconio::input;

fn main() {
    input! {
        n: isize,
        a: isize, b: isize, c: isize,
    }

    let mut ans = 10000;
    for i in 0..10000 {
        for j in 0..(10000 - i) {
            let val = i * a + j * b;
            if n < val || (n - val) % c != 0 {
                continue;
            }

            let res = i + j + ((n - val) / c);
            if ans > res {
                ans = res;
            }
        }
    }

    println!("{}", ans);
}
