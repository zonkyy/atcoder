use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut cnt: HashMap<usize, usize> = HashMap::new();
    cnt.insert(a[0], 1);
    let mut l = 0;
    let mut r = 0;

    while l < n {
        // r を取り込んでいく
        while r + 1 < n
            && ((cnt.len() < k) || (cnt.len() == k && *cnt.get(&a[r + 1]).unwrap_or(&0) > 0))
        {
            r += 1;
            *cnt.entry(a[r]).or_insert(0) += 1;
        }

        // 最長であれば記録
        if ans < r - l + 1 {
            ans = r - l + 1;
        }

        // l を 1 つ右に動かす
        *cnt.get_mut(&a[l]).unwrap() -= 1;
        if *cnt.get(&a[l]).unwrap() == 0 {
            cnt.remove(&a[l]);
        }
        l += 1;
    }

    println!("{}", ans);
}
