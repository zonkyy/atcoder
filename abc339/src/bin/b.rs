use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    };

    let mut grid = vec![vec!['.'; w]; h];
    let turn_right = vec![(h - 1, 0), (0, 1), (1, 0), (0, w - 1)];
    let mut now = (0, 0, 0);
    for _ in 0..n {
        if grid[now.0][now.1] == '.' {
            grid[now.0][now.1] = '#';
            now.2 = (now.2 + 1) % 4;
            now.0 = (now.0 + turn_right[now.2].0) % h;
            now.1 = (now.1 + turn_right[now.2].1) % w;
        } else {
            grid[now.0][now.1] = '.';
            now.2 = (now.2 + 3) % 4;
            now.0 = (now.0 + turn_right[now.2].0) % h;
            now.1 = (now.1 + turn_right[now.2].1) % w;
        }
    }

    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}
