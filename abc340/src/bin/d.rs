use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut abx: [(usize, usize, Usize1); n-1],
    };
    const UPPER: usize = usize::MAX;

    let mut g = HashMap::new();
    for (i, &(a, b, x)) in abx.iter().enumerate() {
        g.entry(i)
            .or_insert(vec![])
            .extend(vec![(i + 1, a), (x, b)]);
    }

    let mut distance = vec![UPPER; n];
    distance[0] = 0;
    let mut heap = BinaryHeap::new();
    for i in 1..n {
        heap.push((Reverse(UPPER), i));
    }
    heap.push((Reverse(0), 0));

    while let Some((Reverse(d), stage)) = heap.pop() {
        if d > distance[stage] || stage >= n - 1 {
            continue;
        }

        for &(next, to_d) in g.get(&stage).unwrap() {
            if distance[stage] + to_d < distance[next] {
                distance[next] = distance[stage] + to_d;
                heap.push((Reverse(distance[next]), next));
            }
        }
    }

    println!("{}", distance[n - 1]);
}
