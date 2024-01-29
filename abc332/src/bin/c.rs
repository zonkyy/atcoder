use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        (n, m): (isize, isize),
        s: Chars,
    };

    // a: 無地、b: ロゴ
    let (mut a, mut b) = (0, 0);
    let mut need_max = 0;

    for c in &s {
        match c {
            '0' => {
                let need = std::cmp::max(0, a - m) + b;
                if need > need_max {
                    need_max = need;
                }
                (a, b) = (0, 0);
            }
            '1' => a += 1,
            '2' => b += 1,
            _ => unreachable!(),
        }
    }

    let need = std::cmp::max(0, a - m) + b;
    if need > need_max {
        need_max = need;
    }

    println!("{}", need_max);
}
