use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
    };
    const MOD: usize = 1000000007;

    let mut prepre = 1;
    let mut pre = 1;
    let mut ans = 1;

    for i in 3..=n {
        ans = prepre + pre;
        ans %= MOD;
        prepre = pre;
        pre = ans;
    }

    println!("{}", ans);
}
