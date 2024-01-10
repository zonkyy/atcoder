use proconio::input;

fn main() {
    input! {
        k: i64,
    }

    if k % 9 != 0 {
        println!("{}", 0);
        return;
    }

    let m = 10_usize.pow(9) + 7;
    let mut dp: Vec<usize> = vec![1];
    for i in 1..=k {
        let mut sum = 0;
        for j in 0.max(i - 9)..i {
            sum = (sum + dp[j as usize]) % m;
        }

        dp.push(sum);
    }

    println!("{}", dp.pop().unwrap());
}
