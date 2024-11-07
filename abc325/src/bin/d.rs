use std::{cmp::Reverse, collections::BinaryHeap};

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
        mut td: [(usize, usize); n],
    };

    for i in 0..n {
        td[i] = (td[i].0, td[i].0 + td[i].1);
    }
    td.sort();

    let mut now = 0;
    let mut ans: usize = 0;
    let mut tdi = 0;
    let mut bh = BinaryHeap::new();
    while tdi < n || !bh.is_empty() {
        // bh に何もなければ時間を進める
        if bh.is_empty() {
            now = td[tdi].0;
        }
        // 現在時刻のものを全部 bh に入れる
        while tdi < n && td[tdi].0 == now {
            bh.push(Reverse(td[tdi].1));
            tdi += 1;
        }
        // 刻印が間に合わなかったものを全て捨てる
        while let Some(&Reverse(top)) = bh.peek() {
            if top < now {
                bh.pop();
            } else {
                break;
            }
        }

        // 刻印が間に合った中で一番猶予が少ないものを捨てて ans += 1
        if let Some(Reverse(top)) = bh.pop() {
            ans += 1;
            now += 1;
        }
    }

    println!("{:?}", ans);
}
