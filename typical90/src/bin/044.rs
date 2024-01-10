use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        txy: [(usize, usize, usize); q],
    }

    let mut queue = VecDeque::from(a);
    for (t, x, y) in txy {
        match t {
            1 => {
                let tmp = queue[x - 1];
                queue[x - 1] = queue[y - 1];
                queue[y - 1] = tmp;
            }
            2 => {
                let val = queue.pop_back().unwrap();
                queue.push_front(val);
            }
            3 => {
                println!("{}", queue[x - 1]);
            }
            _ => (),
        }
    }
}
