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
        k: usize,
    };

    let mut list = Vec::new();
    for i in 1..=10 {
        for mut comb in (0..=9).combinations(i) {
            comb.sort();
            let s = format!("{:0>10}", comb.iter().rev().join(""));
            list.push(s);
        }
    }

    list.sort();
    println!("{}", list[k].trim_start_matches("0"));
}
