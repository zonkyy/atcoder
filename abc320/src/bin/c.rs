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
        m: usize,
        s: [Chars; 3],
    };

    let mut ans = usize::MAX;
    for i in 0..10 {
        let target = ('0' as u8 + i as u8) as char;
        if !(s[0].contains(&target) && s[1].contains(&target) && s[2].contains(&target)) {
            continue;
        }

        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let mut idx = 0;
            for si in order {
                while s[si][idx % m] != target {
                    idx += 1;
                }
                idx += 1;
            }
            ans = ans.min(idx);
        }
    }

    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans - 1);
    }
}
