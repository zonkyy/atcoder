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
        s: String,
        t: String,
    };

    let l = t.find(&s);
    let r = t.rfind(&s);
    let ans = match (l, r) {
        (Some(0), Some(i)) if i == m - n => 0,
        (Some(0), _) => 1,
        (_, Some(i)) if i == m - n => 2,
        _ => 3,
    };
    println!("{}", ans);
}
