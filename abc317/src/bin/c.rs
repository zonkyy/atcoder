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

fn dfs(
    distance: usize,
    now: usize,
    visited: HashSet<usize>,
    roads: &Vec<Vec<(usize, usize)>>,
) -> usize {
    let mut max_distance = distance;

    for &(next, c) in roads[now].iter() {
        if !visited.contains(&next) {
            let mut new_visited = visited.clone();
            new_visited.insert(next);
            max_distance = max_distance.max(dfs(distance + c, next, new_visited, roads));
        }
    }

    return max_distance;
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    };

    let mut roads = vec![vec![]; n];
    for &(a, b, c) in abc.iter() {
        roads[a].push((b, c));
        roads[b].push((a, c));
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(dfs(0, i, HashSet::from([i]), &roads));
    }
    println!("{}", ans);
}
