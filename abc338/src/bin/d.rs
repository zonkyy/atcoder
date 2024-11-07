use proconio::{fastout, input, marker::*};
use std::cmp;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
    };

    // 橋を封鎖しないときの長さ
    let mut min_d = 0;
    for i in 0..(m - 1) {
        let mut d = (x[i] as i64 - x[i + 1] as i64).abs() as usize;
        d = cmp::min(d, n - d);
        min_d += d;
    }

    // <i>-<i+1> 区間を封鎖をどの橋にしたときにいくつ増減するか、を diffs[i] に記載
    let mut diffs: Vec<i64> = vec![0; n];
    for i in 0..(m - 1) {
        let small = cmp::min(x[i], x[i + 1]);
        let big = cmp::max(x[i], x[i + 1]);
        let d = big - small;
        if d < n - d {
            // 小さい方 → 大きい方 の距離が短いため、小～大の封鎖によって距離増加
            let extra = (n - 2 * d) as i64;
            diffs[small] += extra;
            diffs[big] -= extra;
        } else {
            // 小さい方 → 大きい方 の距離が遠いため、0～小 と 大～m-1 の封鎖によって距離増加
            let extra = (2 * d - n) as i64;
            diffs[0] += extra;
            diffs[small] -= extra;
            diffs[big] += extra;
        }
    }

    // diffs を畳み込んだときの最小値を min_d に足したものが答え
    let mut diffs_min = diffs[0];
    for i in 0..(n - 1) {
        diffs[i + 1] += diffs[i];
        diffs_min = diffs_min.min(diffs[i + 1]);
    }

    println!("{}", min_d + diffs_min as usize);
}
