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

fn mark(s: &mut Vec<Vec<char>>, r: usize, c: usize) {
    s[r][c] = '.';
    let d: [(i64, i64); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for &(dr, dc) in d.iter() {
        let next_r = r as i64 + dr;
        let next_c = c as i64 + dc;
        if next_r < 0 || next_r as usize >= s.len() || next_c < 0 || next_c as usize >= s[0].len() {
            continue;
        }

        let next_r = next_r as usize;
        let next_c = next_c as usize;
        if s[next_r][next_c] == '#' {
            mark(s, next_r, next_c);
        }
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    };

    let mut ans = 0;
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == '#' {
                mark(&mut s, r, c);
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
