use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut l: usize,
        r: usize,
    };

    let mut ans = Vec::new();
    while l < r {
        let mut now = l;
        let mut cnt = 0;
        while now % 2 == 0 && (now + 1) * 2usize.pow(cnt) <= r {
            now /= 2;
            cnt += 1;
        }

        while (now + 1) * 2usize.pow(cnt) > r {
            now *= 2;
            cnt -= 1;
        }

        ans.push((l, (now + 1) * 2usize.pow(cnt)));
        l = (now + 1) * 2usize.pow(cnt);
    }

    println!("{}", ans.len());
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
