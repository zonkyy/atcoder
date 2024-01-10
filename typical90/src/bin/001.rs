use proconio::input;

fn solve(a: &Vec<usize>, k: usize, l: usize, m: usize) -> bool {
    let mut cnt = 0;
    let mut now = 0;
    for e in a {
        if e - now >= m {
            now = *e;
            cnt += 1;
        }
    }

    if l - now >= m {
        cnt += 1
    }

    return cnt >= k + 1;
}

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut a: [usize; n],
    }
    let (mut left, mut right): (usize, usize) = (0, l);

    while right - left > 1 {
        let mid: usize = left + ((right - left) / 2);
        if solve(&a, k, l, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
