use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[isize; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    }

    // x と y が仲が悪ければ map[y][x], map[x][y] どちらも true
    let mut map: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for (x, y) in xy {
        map[y - 1][x - 1] = true;
        map[x - 1][y - 1] = true;
    }

    let mut min_time = std::isize::MAX;
    for perm in (0..n).permutations(n) {
        let mut can_select = true;
        let mut time = 0;
        for (i, j) in perm.iter().enumerate() {
            if i < n - 1 && map[*j][perm[i + 1]] == true {
                can_select = false;
                break;
            }
            time += a[*j][i];
        }

        if can_select && time < min_time {
            min_time = time;
        }
    }

    if min_time == std::isize::MAX {
        min_time = -1;
    }

    println!("{}", min_time);
}
