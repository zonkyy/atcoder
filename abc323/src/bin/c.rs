use std::collections::BinaryHeap;

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
        a: [usize; m],
        s: [Chars; n],
    };

    let mut top2 = vec![0; 2];
    for (bonus, line) in s.iter().enumerate() {
        let mut point = bonus;
        for i in 0..m {
            if line[i] == 'o' {
                point += a[i];
            }
        }
        if point > top2[0] {
            top2[1] = top2[0];
            top2[0] = point;
        } else if point > top2[1] {
            top2[1] = point;
        }
    }

    for (bonus, line) in s.iter().enumerate() {
        let mut bh = BinaryHeap::new();
        let mut point = bonus;
        for i in 0..m {
            if line[i] == 'o' {
                point += a[i];
            } else {
                bh.push(a[i]);
            }
        }

        if top2[0] == point {
            println!("0");
            continue;
        }

        let mut ans = 0;
        while point <= top2[0] {
            point += bh.pop().unwrap();
            ans += 1;
        }
        println!("{}", ans);
    }
}
