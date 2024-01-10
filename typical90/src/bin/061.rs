use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    }

    let mut deck = VecDeque::new();
    for (t, x) in tx {
        match t {
            1 => deck.push_front(x),
            2 => deck.push_back(x),
            _ => println!("{}", deck[x - 1]),
        }
    }
}
