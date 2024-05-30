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

fn all_pattern(mut p: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut ret = Vec::new();
    for _ in 0..4 {
        let mut ext_p = vec![vec!['.'; 10]; 10];
        let mut cnt = 0;
        for r in 0..4 {
            for c in 0..4 {
                if p[r][c] == '#' {
                    ext_p[r + 3][c + 3] = p[r][c];
                    cnt += 1;
                }
            }
        }

        for dr in 0..=6 {
            for dc in 0..=6 {
                let mut tmp_cnt = 0;
                let mut tmp_p = vec![vec!['.'; 4]; 4];
                for r in 0..4 {
                    for c in 0..4 {
                        if ext_p[r + dr][c + dc] == '#' {
                            tmp_p[r][c] = ext_p[r + dr][c + dc];
                            tmp_cnt += 1;
                        }
                    }
                }

                if tmp_cnt == cnt {
                    ret.push(tmp_p);
                }
            }
        }

        // 回転
        let mut new_p = vec![vec!['x'; 4]; 4];
        for r in 0..4 {
            for c in 0..4 {
                new_p[r][c] = p[3 - c][r];
            }
        }

        p = new_p;
    }

    return ret.into_iter().unique().collect();
}

#[fastout]
fn main() {
    input! {
        p1: [Chars; 4],
        p2: [Chars; 4],
        p3: [Chars; 4],
    };

    let mut cnt = 0;
    for r in 0..4 {
        for c in 0..4 {
            if p1[r][c] == '#' {
                cnt += 1;
            }
            if p2[r][c] == '#' {
                cnt += 1;
            }
            if p3[r][c] == '#' {
                cnt += 1;
            }
        }
    }

    if cnt != 16 {
        println!("No");
        return;
    }

    let p1_pat = all_pattern(p1);
    let p2_pat = all_pattern(p2);
    let p3_pat = all_pattern(p3);

    for p1 in p1_pat.iter() {
        for p2 in p2_pat.iter() {
            'select_p: for p3 in p3_pat.iter() {
                for r in 0..4 {
                    for c in 0..4 {
                        if ((p1[r][c] == '#') as usize)
                            + ((p2[r][c] == '#') as usize)
                            + ((p3[r][c] == '#') as usize)
                            >= 2
                        {
                            continue 'select_p;
                        }
                    }
                }

                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
