use proconio::input;

fn main() {
    input! {
        n: usize,
        lrxy: [(usize, usize, usize, usize); n],
    }
    let mut field: Vec<Vec<i64>> = vec![vec![0; 1001]; 1001];
    for (lx, ly, rx, ry) in lrxy {
        field[ly][lx] += 1;
        field[ry][rx] += 1;
        field[ly][rx] -= 1;
        field[ry][lx] -= 1;
    }

    for r in 0..=1000 {
        for c in 1..=1000 {
            field[r][c] += field[r][c - 1];
        }
    }

    for c in 0..=1000 {
        for r in 1..=1000 {
            field[r][c] += field[r - 1][c];
        }
    }

    let mut cnt: Vec<usize> = vec![0; n + 1];
    for r in field {
        for c in r {
            cnt[c as usize] += 1;
        }
    }

    println!(
        "{}",
        &cnt[1..]
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}
