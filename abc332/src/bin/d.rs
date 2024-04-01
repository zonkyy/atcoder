use itertools::Itertools;
use proconio::input;

fn count_swap(a: &Vec<usize>) -> usize {
    let mut cnt = 0;
    let mut ary = a.to_vec();
    for _ in 0..ary.len() {
        for i in 0..(ary.len() - 1) {
            if ary[i] > ary[i + 1] {
                cnt += 1;
                let tmp = ary[i];
                ary[i] = ary[i + 1];
                ary[i + 1] = tmp;
            }
        }
    }

    cnt
}

fn main() {
    input! {
        (h, w): (usize, usize),
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    };

    for h_perm in (0..h).permutations(h) {
        for w_perm in (0..w).permutations(w) {
            let mut ok = true;
            for i in 0..h {
                for j in 0..w {
                    if a[i][j] != b[h_perm[i]][w_perm[j]] {
                        ok = false;
                        break;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                //println!("{:?} {:?}", w_perm, h_perm);
                println!("{}", count_swap(&w_perm) + count_swap(&h_perm));
                return;
            }
        }
    }

    println!("-1");
}
