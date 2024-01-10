use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let sum = a
        .iter()
        .zip(b)
        .fold(0, |acc, (av, bv)| acc + (av - bv).abs());

    let cond = sum <= k && k % 2 == sum % 2;
    println!("{}", if cond { "Yes" } else { "No" });
}
