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
        d: usize,
        p: usize,
        mut f: [usize; n],
    };

    f.sort();

    let mut ans = 0;
    while f.len() >= d {
        let mut tmp = f.split_off(f.len() - d);
        let sum = tmp.iter().sum::<usize>();
        if sum <= p {
            ans += sum;
            break;
        }
        ans += p;
    }

    let rest_sum = f.iter().sum::<usize>();
    if f.len() < d && p < rest_sum {
        ans += p;
    } else {
        ans += rest_sum;
    }

    println!("{}", ans);
}
