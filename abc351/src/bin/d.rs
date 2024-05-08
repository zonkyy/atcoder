use std::cmp;

use im_rc::HashSet;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    let mut field = vec![vec!['.'; w + 2]; h + 2];
    for r in 0..h + 2 {
        for c in 0..w + 2 {
            if r == 0 || c == 0 || r == h + 1 || c == w + 1 {
                field[r][c] = '-';
            } else if s[r - 1][c - 1] == '#' {
                field[r][c] = '#';
            }
        }
    }

    for r in 1..h + 1 {
        for c in 1..w + 1 {
            if field[r][c] == '.'
                && vec![
                    field[r - 1][c],
                    field[r + 1][c],
                    field[r][c - 1],
                    field[r][c + 1],
                ]
                .contains(&'#')
            {
                field[r][c] = 'x';
            }
        }
    }

    // for r in 0..h + 2 {
    //     for c in 0..w + 2 {
    //         print!("{}", field[r][c]);
    //     }
    //     println!();
    // }

    let mut ans = 1;
    let mut checked = vec![vec![false; w + 2]; h + 2];
    for r in 0..h + 2 {
        for c in 0..w + 2 {
            if field[r][c] == '.' && !checked[r][c] {
                let mut queue = vec![(r, c)];
                checked[r][c] = true;
                let mut can_move: HashSet<(usize, usize)> = queue.iter().cloned().collect();
                while let Some(cell) = queue.pop() {
                    for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let nr = (cell.0 as i64 + d.0) as usize;
                        let nc = (cell.1 as i64 + d.1) as usize;
                        match field[nr][nc] {
                            '.' => {
                                if !checked[nr][nc] {
                                    can_move.insert((nr, nc));
                                    queue.push((nr, nc));
                                    checked[nr][nc] = true;
                                }
                            }
                            'x' => {
                                can_move.insert((nr, nc));
                                ()
                            }
                            _ => (),
                        };
                    }
                }

                ans = cmp::max(ans, can_move.len());
            }
        }
    }

    println!("{}", ans);
}
