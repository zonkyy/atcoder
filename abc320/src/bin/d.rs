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
        m: usize,
        abxy: [(Usize1, Usize1, i64, i64); m],
    };

    let mut adj = vec![vec![]; n];
    for &(a, b, x, y) in &abxy {
        adj[a].push((b, x, y));
        adj[b].push((a, -x, -y));
    }

    let mut ans = vec![(i64::MAX, i64::MAX); n];
    ans[0] = (0, 0);
    let mut rest = vec![0];
    while !rest.is_empty() {
        let next = rest.pop().unwrap();
        for &(node, x, y) in adj[next].iter() {
            if ans[node].0 == i64::MAX {
                ans[node] = (ans[next].0 + x, ans[next].1 + y);
                rest.push(node);
            }
        }
    }

    for &(x, y) in ans.iter() {
        if x == i64::MAX {
            println!("undecidable");
        } else {
            println!("{} {}", x, y);
        }
    }
}
