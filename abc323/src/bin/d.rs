use std::collections::HashMap;

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
        sc: [(usize, usize); n],
    };

    let mut sc2 = HashMap::new();
    for &(s, c) in sc.iter() {
        let mut m = 1;
        let mut s2 = s;
        while s2 % 2 == 0 {
            s2 /= 2;
            m *= 2;
        }

        *sc2.entry(s2).or_insert(0) += m * c;
    }

    let mut ans = 0;
    for (&k, &v) in sc2.iter() {
        let mut v2 = v;
        let mut i = 1;
        while i < v2 {
            i *= 2;
        }

        while v2 > 0 {
            ans += v2 / i;
            v2 %= i;
            i /= 2;
        }
    }

    println!("{}", ans);
}
