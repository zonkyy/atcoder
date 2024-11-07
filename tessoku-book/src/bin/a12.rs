use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut l = 0;
    let mut r = 10usize.pow(9) + 9;
    while r - l > 1 {
        let now = (l + r) / 2;
        let mut cnt = 0;
        for &i in a.iter() {
            cnt += now / i;
        }

        //println!("{} {}, {}", l, r, cnt);

        if cnt >= k {
            r = now;
        } else {
            l = now;
        }
    }

    let mut l_cnt = 0;
    for &i in a.iter() {
        l_cnt += l / i;
    }
    let mut r_cnt = 0;
    for &i in a.iter() {
        r_cnt += r / i;
    }

    if k <= l_cnt {
        println!("{}", l);
    } else {
        println!("{}", r);
    }
}
