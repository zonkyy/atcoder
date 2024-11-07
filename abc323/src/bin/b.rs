use itertools::Itertools;
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
        s: [Chars; n],
    };

    let ans = s
        .iter()
        .enumerate()
        .sorted_by(|(_, a), (_, b)| {
            Ord::cmp(
                &b.iter().counts().get(&'o').unwrap_or(&0),
                &a.iter().counts().get(&'o').unwrap_or(&0),
            )
        })
        .map(|(i, _)| i + 1)
        .join(" ");
    println!("{}", ans);
}
