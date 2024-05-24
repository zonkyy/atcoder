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
        n: u32,
        mut s: String,
    };
    s = s.chars().sorted().collect::<String>();
    let UPPER: usize = 10usize.pow(n);

    let mut ans = 0;
    let mut i = 0;
    while i * i < UPPER {
        let t = format!("{:0>width$}", i * i, width = n as usize)
            .chars()
            .sorted()
            .collect::<String>();
        if s == t {
            ans += 1;
        }
        i += 1;
    }

    println!("{}", ans);
}
