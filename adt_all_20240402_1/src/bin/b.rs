use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let mut ans = 1;
    for _ in b..a {
        ans *= 32;
    }

    println!("{}", ans);
}
