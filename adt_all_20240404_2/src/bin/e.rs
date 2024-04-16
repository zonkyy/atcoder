use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        (h, w): (i64, i64),
        g: [Chars; h],
    };

    let mut now = (0, 0);
    for _ in 0..1000000 {
        let next: (i64, i64) = match g[now.0][now.1] {
            'U' => (now.0 as i64 - 1, now.1 as i64),
            'D' => (now.0 as i64 + 1, now.1 as i64),
            'L' => (now.0 as i64, now.1 as i64 - 1),
            'R' => (now.0 as i64, now.1 as i64 + 1),
            _ => unreachable!(),
        };
        if next.0 < 0 || next.0 >= h || next.1 < 0 || next.1 >= w {
            println!("{} {}", now.0 + 1, now.1 + 1);
            return;
        }
        now = (next.0 as usize, next.1 as usize);
    }

    println!("-1");
}
