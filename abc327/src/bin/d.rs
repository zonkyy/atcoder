use std::collections::HashSet;

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    };

    let mut adj = vec![HashSet::<usize>::new(); n];
    let mut unchecked = HashSet::new();
    for (&ai, &bi) in a.iter().zip(b.iter()) {
        adj[ai].insert(bi);
        adj[bi].insert(ai);
        unchecked.insert(ai);
        unchecked.insert(bi);
    }

    let mut numbers = vec![-1; n];
    let mut queue = Vec::new();
    queue.push(unchecked.take(&a[0]).unwrap());
    while !(unchecked.is_empty() && queue.is_empty()) {
        // 次に処理するノード取得
        let next_node;
        if let Some(node) = queue.pop() {
            next_node = node;
        } else {
            let node = *unchecked.iter().next().unwrap();
            next_node = unchecked.take(&node).unwrap();
        }

        // まだ数字が決まってないなら 0 を設定
        if numbers[next_node] == -1 {
            numbers[next_node] = 0;
        }

        // 次のノードと隣接しているノードの数字が矛盾したら終了
        // 矛盾していないものは queue に入れていく
        for &nextto in adj[next_node].iter() {
            if numbers[nextto] == numbers[next_node] {
                println!("No");
                return;
            }

            if numbers[nextto] == -1 {
                numbers[nextto] = 1 - numbers[next_node];
            }

            if unchecked.contains(&nextto) {
                queue.push(nextto);
                unchecked.take(&nextto);
                numbers[nextto] = 1 - numbers[next_node];
            }
        }
    }

    println!("Yes");
}
