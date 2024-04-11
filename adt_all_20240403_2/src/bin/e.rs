use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2*n],
    };

    let mut win_cnt: Vec<usize> = vec![0; 2 * n];
    for round in 0..m {
        // println!("win_cnt: {:?}", win_cnt);
        let rank_sorted: Vec<(usize, usize)> = win_cnt
            .iter()
            .enumerate()
            .map(|(i, &count)| (i, count))
            .sorted_by(|a, b| b.1.cmp(&a.1))
            .collect();
        for pair in rank_sorted.chunks(2) {
            if (a[pair[0].0][round] == 'G' && a[pair[1].0][round] == 'C')
                || (a[pair[0].0][round] == 'P' && a[pair[1].0][round] == 'G')
                || (a[pair[0].0][round] == 'C' && a[pair[1].0][round] == 'P')
            {
                win_cnt[pair[0].0] += 1;
                // println!("{:?} vs {:?} = win {:?}", pair[0].0, pair[1].0, pair[0].0);
            } else if (a[pair[0].0][round] == 'G' && a[pair[1].0][round] == 'P')
                || (a[pair[0].0][round] == 'P' && a[pair[1].0][round] == 'C')
                || (a[pair[0].0][round] == 'C' && a[pair[1].0][round] == 'G')
            {
                win_cnt[pair[1].0] += 1;
                // println!("{:?} vs {:?} = win {:?}", pair[0].0, pair[1].0, pair[1].0);
            } else {
                // println!("{:?} vs {:?} = draw", pair[0].0, pair[1].0);
            }
        }
    }

    // println!("{:?}", win_cnt);
    for win in (0..=m).rev() {
        for (i, &cnt) in win_cnt.iter().enumerate() {
            if win == cnt {
                println!("{}", i + 1);
            }
        }
    }
}
