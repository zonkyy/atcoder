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
        xyz: [(usize, usize, usize); n],
    };
    let need_seat = xyz.iter().map(|&(_, _, z)| z).sum::<usize>() / 2 + 1;

    let mut dp = vec![vec![usize::MAX; need_seat + 1]; n + 1];
    dp[0][0] = 0;

    for (r, &(x, y, z)) in xyz.iter().enumerate() {
        let need;
        if x > y {
            need = 0;
        } else {
            need = (y - x + 1) / 2;
        }

        for i in 0..=need_seat {
            let idx = if z <= i { i - z } else { 0 };
            if dp[r][idx] == usize::MAX {
                break;
            }
            dp[r + 1][i] = dp[r][i].min(dp[r][idx] + need);
        }
    }

    println!("{}", dp[n][need_seat]);
}
