use std::collections::BTreeSet;

use proconio::input;

// BTreeSet を使用
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        q: usize,
        b: [isize; q],
    }
    let set: BTreeSet<isize> = a.into_iter().collect();

    for bi in b {
        let left = set.range(..bi).next_back();
        let right = set.range(bi..).next();

        let left_score = match left {
            Some(x) => (bi - x).abs(),
            None => std::isize::MAX,
        };
        let right_score = match right {
            Some(x) => (bi - x).abs(),
            None => std::isize::MAX,
        };
        println!("{}", left_score.min(right_score));
    }
}

// 自前で二分探索
// fn main() {
//     input! {
//         n: usize,
//         mut a: [isize; n],
//         q: usize,
//         b: [isize; q],
//     }

//     a.sort();

//     for bi in b {
//         let (mut left, mut right): (usize, usize) = (0, n - 1);
//         while right - left >= 2 {
//             let middle = left + ((right - left) / 2);
//             if bi <= a[middle] {
//                 right = middle;
//             } else {
//                 left = middle;
//             }
//         }

//         let l_val = (a[left] - bi).abs();
//         let r_val = (a[right] - bi).abs();
//         println!("{}", l_val.min(r_val));
//     }
// }
