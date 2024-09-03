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

fn is_multiple(n: usize, i: usize) -> bool {
    if n == 0 {
        return true;
    }

    n % i == 0
}

#[fastout]
fn main() {
    input! {
        n: usize,
    };

    'outer: for i in 0..=n {
        for j in 1..=9 {
            if n % j == 0 && is_multiple(i, n / j) {
                print!("{}", j);
                continue 'outer;
            }
        }
        print!("-")
    }

    println!();
}
