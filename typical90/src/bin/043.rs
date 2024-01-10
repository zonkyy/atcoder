use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        (h, w): (usize, usize),
        (rs, cs): (usize, usize),
        (rt, ct): (usize, usize),
        s: [String; h],
    }
    let max = 9999999;

    // 壁 -1、道 max、周囲を壁で囲った field 作成
    let mut field: Vec<Vec<i64>> = vec![vec![-1; w + 2]; h + 2];
    for r in 0..h {
        let vec_char: Vec<char> = s[r].chars().collect();
        for c in 0..w {
            if vec_char[c] == '.' {
                field[r + 1][c + 1] = max;
            }
        }
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    field[rs][cs] = -1;
    queue.push_front((rs, cs));
    while let Some((r, c)) = queue.pop_back() {
        // 4 方向直進した道を埋める
        // 上
        let mut step = 1;
        while field[r - step][c] != -1 {
            if field[r][c] + 1 < field[r - step][c] {
                field[r - step][c] = field[r][c] + 1;
                queue.push_front((r - step, c));
            }
            step += 1;
        }

        // 下
        step = 1;
        while field[r + step][c] != -1 {
            if field[r][c] + 1 < field[r + step][c] {
                field[r + step][c] = field[r][c] + 1;
                queue.push_front((r + step, c));
            }
            step += 1;
        }

        // 左
        step = 1;
        while field[r][c - step] != -1 {
            if field[r][c] + 1 < field[r][c - step] {
                field[r][c - step] = field[r][c] + 1;
                queue.push_front((r, c - step));
            }
            step += 1;
        }

        // 右
        step = 1;
        while field[r][c + step] != -1 {
            if field[r][c] + 1 < field[r][c + step] {
                field[r][c + step] = field[r][c] + 1;
                queue.push_front((r, c + step));
            }
            step += 1;
        }

        if field[rt][ct] != max {
            println!("{}", field[rt][ct]);
            return;
        }
    }

    field[rs][cs] = 0;
    println!("{}", field[rt][ct]);
}
