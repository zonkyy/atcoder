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

#[fastout]
fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    };

    let mut ans = Vec::new();
    for (idx, l) in s.iter().enumerate() {
        // 同じ
        if l == &t {
            ans.push(idx + 1);
            continue;
        }

        let mut diff = 0;
        if l.len() == t.len() + 1 {
            // 一文字挿入
            let mut li = 0;
            let mut ti = 0;
            while li < l.len() || ti < t.len() {
                if li >= l.len() {
                    ti += 1;
                    diff += 1;
                } else if ti >= t.len() {
                    li += 1;
                    diff += 1;
                } else if l[li] != t[ti] {
                    li += 1;
                    diff += 1;
                } else {
                    li += 1;
                    ti += 1;
                }
            }
        } else if l.len() == t.len() - 1 {
            // 一文字削除
            let mut li = 0;
            let mut ti = 0;
            while li < l.len() || ti < t.len() {
                if li >= l.len() {
                    ti += 1;
                    diff += 1;
                } else if ti >= t.len() {
                    li += 1;
                    diff += 1;
                } else if l[li] != t[ti] {
                    ti += 1;
                    diff += 1;
                } else {
                    li += 1;
                    ti += 1;
                }
            }
        } else if l.len() == t.len() {
            // 1 文字変更
            for i in 0..l.len() {
                if l[i] != t[i] {
                    diff += 1;
                }
            }
        }

        if diff == 1 {
            ans.push(idx + 1);
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
