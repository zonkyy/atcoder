use std::collections::VecDeque;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    };
    const UPPER: usize = 5000 * 10usize.pow(9);

    let mut deque = VecDeque::new();
    for &c in s.iter() {
        deque.push_back(c);
    }

    let mut ans = UPPER;
    for mv in 0..n {
        let mut diff = 0;
        for i in 0..n {
            if deque[i] != deque[n - i - 1] {
                diff += 1;
            }
        }

        ans = ans.min(mv * a + diff / 2 * b);

        let c = deque.pop_front().unwrap();
        deque.push_back(c);
    }

    println!("{}", ans);
}
