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
        s: String,
    };
    const UPPER: usize = 3162300;

    if s == "0".to_string() {
        println!("1");
        return;
    }

    // (1～9 のソート、0 の数)
    let mut sqs = Vec::new();
    for i in 1..UPPER {
        let s_num = (i * i).to_string();
        let mut s_num_nonzero: Vec<char> = s_num.replace("0", "").chars().collect();
        s_num_nonzero.sort();
        let s_num_nonzero: String = s_num_nonzero.into_iter().collect();
        let zero_cnt = s_num.len() - s_num_nonzero.len();
        sqs.push((s_num_nonzero, zero_cnt));
    }

    let mut s_nonzero = s.replace("0", "").chars().collect::<Vec<_>>();
    s_nonzero.sort();
    let s_nonzero = s_nonzero.into_iter().collect::<String>();
    let s_zero_cnt = n - s_nonzero.len();
    let ans = sqs
        .iter()
        .filter(|x| x.0 == s_nonzero && x.1 <= s_zero_cnt)
        .count();
    println!("{}", ans);
}
