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
        mut h: [i64; n],
    };

    let mut cnt = 1;
    for target in h.iter() {
        cnt += target / 5 * 3;
        let mut rest = target % 5;
        while rest > 0 {
            if cnt % 3 == 0 {
                rest -= 3;
            } else {
                rest -= 1;
            }
            cnt += 1;
        }
    }

    println!("{}", cnt - 1);
}
