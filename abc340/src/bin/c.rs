use std::collections::HashMap;

use num_integer::div_ceil;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: u128,
    };

    let mut v = vec![n];
    let mut memo = HashMap::new();

    let mut ans = 0;
    while let Some(i) = v.pop() {
        if let Some(&(ceil, floor)) = memo.get(&i) {
            if ceil >= 2 {
                v.push(ceil);
            }
            if floor >= 2 {
                v.push(floor);
            }
            ans += i;
        } else {
            let ceil = i / 2 + if i % 2 == 0 { 0 } else { 1 };
            let floor = i / 2;
            memo.insert(i, (ceil, floor));
            if ceil >= 2 {
                v.push(ceil);
            }
            if floor >= 2 {
                v.push(floor);
            }
            ans += i;
        }
    }

    println!("{}", ans);
}
