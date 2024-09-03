use bitvec::vec;
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
        c: [usize; 9],
    };

    let indices: Vec<usize> = (0..9).collect::<Vec<_>>();
    let pattern = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
        vec![0, 3, 6],
        vec![1, 4, 7],
        vec![2, 5, 8],
        vec![0, 4, 8],
        vec![2, 4, 6],
    ];
    let mut bad_cnt = 0;

    for perm in indices.iter().permutations(indices.len()) {
        // perm[k] = v は「k 番目に c[v] を見る」の意
        let mut order = vec![0; 9];
        for i in 0..9 {
            // order[k] = v は「c[k] の値が v 番目に見られる」の意
            order[*perm[i]] = i;
        }

        for p in pattern.iter() {
            // p.0 は order[p.0] 番目に見られる、p.1/p.2 も同様
            // すなわち order[p.0], order[p.1], order[p.2] のうち一番大きいものが残り 2 つと異なると bad
            let max_order_idx = p.iter().max_by_key(|&x| order[*x]).unwrap();
            if c[p[0]] == c[p[1]] && max_order_idx == &p[2]
                || c[p[1]] == c[p[2]] && max_order_idx == &p[0]
                || c[p[2]] == c[p[0]] && max_order_idx == &p[1]
            {
                bad_cnt += 1;
                break;
            }
        }
    }

    println!(
        "{}",
        1.0 - bad_cnt as f64 / (1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9) as f64
    );
}
