use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: isize,
        q: usize,
        queries: [(usize, String); q],
    }

    let mut head = (1, 0);
    let mut history: VecDeque<(isize, isize)> = VecDeque::new();
    for i in 1..=n {
        history.push_back((i, 0));
    }

    for (t, x) in queries {
        match t {
            1 => {
                match x.as_str() {
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    "L" => head.0 -= 1,
                    "R" => head.0 += 1,
                    _ => unreachable!(),
                }
                history.push_front(head);
            }
            2 => {
                let node = history[x.parse::<usize>().unwrap() - 1];
                println!("{} {}", node.0, node.1);
            }
            _ => unreachable!(),
        }
    }
}
