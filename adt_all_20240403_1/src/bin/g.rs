use proconio::{fastout, input, marker::*};

const MEMO_SIZE: usize = 100000000;

fn f(memo: &mut Vec<u128>, n: u128) -> u128 {
    if n >= MEMO_SIZE as u128 {
        return f(memo, n / 2) + f(memo, n / 3);
    } else {
        if memo[n as usize] == 0 {
            memo[n as usize] = f(memo, n / 2) + f(memo, n / 3);
            return memo[n as usize];
        } else {
            return memo[n as usize];
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: u128,
    };

    let mut ans = 0;
    let mut memo = vec![0; MEMO_SIZE];
    memo[0] = 1;

    println!("{:?}", f(&mut memo, n));
}
