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
        p: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    };
    a.sort();
    b.sort();

    let mut acc = vec![0];
    acc.extend(b.clone());
    for i in 1..(m + 1) {
        acc[i] += acc[i - 1];
    }

    let mut ans = 0;
    for ai in 0..n {
        if a[ai] >= p {
            ans += p * m;
        } else {
            let bi = b.partition_point(|&i| i <= p - a[ai]);
            ans += a[ai] * bi + acc[bi];
            if bi < m {
                ans += p * (m - bi);
            }
        }
    }

    println!("{}", ans);
}
