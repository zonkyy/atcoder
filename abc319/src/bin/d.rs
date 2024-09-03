use std::usize;

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

fn cnt_line(width: usize, l: &[usize]) -> usize {
    let mut now_width = 0;
    let mut line = 1;
    for &w in l.iter() {
        now_width += w;
        if now_width > width {
            line += 1;
            now_width = w;
        }
        now_width += 1;
    }

    line
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: [usize; n],
    };

    let mut left = *l.iter().max().unwrap();
    let mut right = l.iter().sum::<usize>() + n;

    let mut min_width = usize::MAX;
    if cnt_line(left, &l) <= m {
        min_width = left;
    }
    if cnt_line(right, &l) <= m {
        min_width = right;
    }

    while left < right {
        let mid = (left + right) / 2;
        if cnt_line(mid, &l) <= m {
            min_width = mid;
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    println!("{}", min_width);
}
