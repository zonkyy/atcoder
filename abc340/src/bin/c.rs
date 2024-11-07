use std::collections::HashMap;

use proconio::{fastout, input, marker::*};

fn dfs(memo: &mut HashMap<u128, u128>, n: u128) -> u128 {
    if let Some(&i) = memo.get(&n) {
        return i;
    }

    let mut ans = 0;
    let ceil = n / 2 + if n % 2 == 0 { 0 } else { 1 };
    let floor = n / 2;

    if ceil >= 2 {
        ans += dfs(memo, ceil);
    }
    if floor >= 2 {
        ans += dfs(memo, floor);
    }
    ans += n;

    memo.insert(n, ans);
    ans
}

#[fastout]
fn main() {
    input! {
        n: u128,
    };

    let mut memo = HashMap::new();
    println!("{}", dfs(&mut memo, n));
}
