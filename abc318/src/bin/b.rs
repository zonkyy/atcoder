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
        abcd: [(usize, usize, usize, usize); n],
    };

    // field[0][0] = 1 とは、(0,0) を左下とする正方形にシートが掛かっていることを示す
    let mut field = vec![vec![0; 101]; 101];
    for (a, b, c, d) in abcd {
        for i in a..b {
            for j in c..d {
                field[i][j] += 1;
            }
        }
    }

    let mut ans = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            if field[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
