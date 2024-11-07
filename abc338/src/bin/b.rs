use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    };
    s.sort();

    let mut max = '-';
    let mut max_cnt = 0;
    let mut now = '-';
    let mut now_cnt = 0;
    for c in s {
        if c != now {
            now = c;
            now_cnt = 0;
        }

        now_cnt += 1;

        if now_cnt > max_cnt {
            max = now;
            max_cnt = now_cnt;
        }
    }

    println!("{}", max);
}
