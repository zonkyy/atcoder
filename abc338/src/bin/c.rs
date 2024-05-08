use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: [usize; n],
        a: [usize; n],
        b: [usize; n],
    };
    const INF: usize = usize::MAX;

    let mut a_cnt_max = INF;
    let mut b_cnt_max = INF;
    for i in 0..n {
        if a[i] != 0 {
            a_cnt_max = a_cnt_max.min(q[i] / a[i]);
        }
        if b[i] != 0 {
            b_cnt_max = b_cnt_max.min(q[i] / b[i]);
        }
    }

    let mut a_cnt = a_cnt_max + 1;
    let mut b_cnt = 0;
    let mut ans = a_cnt_max;
    while a_cnt > 0 {
        a_cnt -= 1;

        'cp: loop {
            b_cnt += 1;
            for i in 0..n {
                if a[i] * a_cnt + b[i] * b_cnt > q[i] {
                    b_cnt -= 1;
                    break 'cp;
                }
            }
        }

        ans = ans.max(a_cnt + b_cnt);
    }

    println!("{}", ans);
}
