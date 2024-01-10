use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
    }

    let m = 10usize.pow(9) + 7;
    let mut dp: Vec<usize> = vec![1];
    for i in 1..=n {
        if i < l {
            dp.push(dp[i - 1] % m);
        } else {
            dp.push((dp[i - 1] + dp[i - l]) % m);
        }
    }

    println!("{}", dp[n]);
}
