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
        wx: [(usize, usize); n],
    };

    let mut attend = vec![vec![0; 24]; n];
    for (i, &(w, x)) in wx.iter().enumerate() {
        for h in 0..24 {
            if (h + x) % 24 >= 9 && (h + x) % 24 <= 17 {
                attend[i][h] = w;
            }
        }
    }

    for i in 1..n {
        for h in 0..24 {
            attend[i][h] += attend[i - 1][h];
        }
    }

    let mut ans = 0;
    for h in 0..24 {
        ans = ans.max(attend[n - 1][h]);
    }

    println!("{}", ans);
}
