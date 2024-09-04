use itertools::Itertools;
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

fn get_zero_positions(bits: usize, len: usize) -> Vec<usize> {
    let mut pos = vec![];
    for i in 0..len {
        if (bits >> i) & 1 == 0 {
            pos.push(i);
        }
    }
    pos
}

#[fastout]
fn main() {
    input! {
        n: usize,
        _d: [usize; n * (n-1) / 2],
    };

    let mut d = vec![vec![0; n]; n];
    let mut i = 0;
    for r in 0..n {
        for c in (r + 1)..n {
            d[r][c] = _d[i];
            i += 1;
        }
    }

    let mut dp = vec![0; 1 << n];
    dp[0] = 0;
    for b in 0..(1 << n) {
        if (b as usize).count_ones() % 2 != 0 {
            continue;
        }

        for comb in get_zero_positions(b, n).iter().combinations(2) {
            let idx = b | (1 << comb[0]) | (1 << comb[1]);
            dp[idx] = dp[idx].max(dp[b] + d[*comb[0]][*comb[1]]);
        }
    }

    println!("{}", dp.iter().max().unwrap());
}
