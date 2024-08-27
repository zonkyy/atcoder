use std::collections::HashSet;

use proconio::{fastout, input, marker::*};

trait Transpose<Iter: IntoIterator> {
    fn transpose(self) -> Transposed<Iter>;
}

impl<T> Transpose<T::Item> for T
where
    T: IntoIterator,
    T::Item: IntoIterator,
{
    fn transpose(self) -> Transposed<T::Item> {
        Transposed(self.into_iter().map(IntoIterator::into_iter).collect())
    }
}

struct Transposed<Iter: IntoIterator>(Vec<Iter::IntoIter>);

impl<Iter: IntoIterator> Iterator for Transposed<Iter> {
    type Item = Vec<Iter::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter_mut().map(Iterator::next).collect()
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(Usize1, Usize1); n-1],
        v: [Usize1; k],
    };
    let v = v.into_iter().collect::<HashSet<usize>>();

    let mut adj = vec![HashSet::new(); n];
    let mut adj_cnt = vec![0; n];
    for &(a, b) in ab.iter() {
        adj[a].insert(b);
        adj[b].insert(a);
        adj_cnt[a] += 1;
        adj_cnt[b] += 1;
    }

    let mut leaves = HashSet::new();
    for (i, &x) in adj_cnt.iter().enumerate() {
        if x == 1 {
            leaves.insert(i);
        }
    }

    leaves = leaves.difference(&v).cloned().collect::<HashSet<usize>>();
    while let Some(&target) = leaves.iter().next() {
        leaves.remove(&target);
        let parent = *adj[target].iter().next().unwrap();
        adj[target].remove(&parent);
        adj[parent].remove(&target);
        if adj[parent].len() == 1 {
            leaves.insert(parent);
        }
        leaves = leaves.difference(&v).cloned().collect::<HashSet<usize>>();
    }

    let ans = adj.iter().filter(|&x| x.len() > 0).count().max(1);
    println!("{}", ans);
}
