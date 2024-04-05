use std::vec;

use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [proconio::marker::Chars; n],
    };

    let mut ans = "".to_string();
    let direction = vec![
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    // どこから始めるか
    for ri in 0..n {
        for ci in 0..n {
            // どの方向に進むか
            for (dr, dc) in direction.iter() {
                let mut now_point = (ri, ci);
                let mut route = vec![];
                // n 回進む
                for _ in 0..n {
                    route.push(a[now_point.0 % n][now_point.1 % n]);
                    now_point.0 = ((now_point.0 + n) as i32 + dr) as usize;
                    now_point.1 = ((now_point.1 + n) as i32 + dc) as usize;
                }
                // 文字列を作成し、最大か判断
                let route_str: String = route.iter().collect();
                if route_str > ans {
                    ans = route_str;
                }
            }
        }
    }

    println!("{}", ans);
}
