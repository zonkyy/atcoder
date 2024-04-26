use std::{cmp, collections::VecDeque};

use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    const INF: usize = usize::MAX;

    let mut p1 = (100, 100);
    let mut p2 = (100, 100);
    for (ri, row) in s.iter().enumerate() {
        for (ci, c) in row.iter().enumerate() {
            if *c == 'P' {
                if p1.0 == 100 {
                    p1 = (ri, ci);
                } else {
                    p2 = (ri, ci);
                }
            }
        }
    }

    // memo[a][b][c][d] = (a,b) と (c,d) の距離のメモ
    let mut memo = vec![vec![vec![vec![INF; n]; n]; n]; n];
    memo[p1.0][p1.1][p2.0][p2.1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((p1.0, p1.1, p2.0, p2.1));
    while let Some((r1, c1, r2, c2)) = queue.pop_front() {
        for (rd, cd) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let mut nr1 = r1 as i64 + rd;
            let mut nc1 = c1 as i64 + cd;
            if nr1 < 0
                || nr1 >= n as i64
                || nc1 < 0
                || nc1 >= n as i64
                || s[nr1 as usize][nc1 as usize] == '#'
            {
                nr1 = r1 as i64;
                nc1 = c1 as i64;
            }

            let mut nr2 = r2 as i64 + rd;
            let mut nc2 = c2 as i64 + cd;
            if nr2 < 0
                || nr2 >= n as i64
                || nc2 < 0
                || nc2 >= n as i64
                || s[nr2 as usize][nc2 as usize] == '#'
            {
                nr2 = r2 as i64;
                nc2 = c2 as i64;
            }

            let nr1 = nr1 as usize;
            let nc1 = nc1 as usize;
            let nr2 = nr2 as usize;
            let nc2 = nc2 as usize;
            if memo[nr1][nc1][nr2][nc2] == INF {
                memo[nr1][nc1][nr2][nc2] = memo[r1][c1][r2][c2] + 1;
                queue.push_back((nr1, nc1, nr2, nc2))
            }
        }
    }

    let mut ans = INF;
    for r1 in 0..n {
        for c1 in 0..n {
            for r2 in 0..n {
                for c2 in 0..n {
                    if (r1, c1) == (r2, c2) {
                        ans = ans.min(memo[r1][c1][r2][c2]);
                    }
                }
            }
        }
    }

    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
