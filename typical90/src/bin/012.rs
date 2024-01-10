use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        h: i64,
        w: i64,
        q: usize,
    }

    let mut field: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];
    let mut uf: UnionFind<usize> = UnionFind::new((h * w) as usize);

    for _ in 0..q {
        input! {
            qn: usize,
        }
        match qn {
            1 => {
                input! {
                    mut r: i64,
                    mut c: i64,
                }
                r -= 1;
                c -= 1;

                field[r as usize][c as usize] = true;
                for (rr, cc) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                    if r + rr < 0 || r + rr >= h as i64 || c + cc < 0 || c + cc >= w as i64 {
                        continue;
                    }

                    if field[(r + rr) as usize][(c + cc) as usize] {
                        uf.union((r * w + c) as usize, ((r + rr) * w + (c + cc)) as usize);
                    }
                }
            }
            2 => {
                input! {
                    mut ra: i64,
                    mut ca: i64,
                    mut rb: i64,
                    mut cb: i64,
                }
                ra -= 1;
                ca -= 1;
                rb -= 1;
                cb -= 1;

                if field[ra as usize][ca as usize]
                    && field[rb as usize][cb as usize]
                    && uf.find((ra * w + ca) as usize) == uf.find((rb * w + cb) as usize)
                {
                    println!("{}", "Yes");
                } else {
                    println!("{}", "No");
                }
            }
            _ => (),
        }
    }
}
