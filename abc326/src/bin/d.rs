use itertools::Itertools;
use proconio::{fastout, input, marker::*};

fn get_lines(len: usize, key: char) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    let base = [vec!['A', 'B', 'C'], vec!['.'; len - 3]].concat();
    for comb in (0..len).permutations(len) {
        if comb
            .iter()
            .filter(|&i| i <= &2)
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            == &&(key as usize - 'A' as usize)
        {
            result.push(comb.iter().cloned().map(|x| base[x]).collect::<Vec<char>>());
        }
    }

    return result.iter().cloned().unique().collect::<Vec<Vec<char>>>();
}

#[fastout]
fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars,
    };

    let mut lines_list = Vec::new();
    for &c in r.iter() {
        lines_list.push(get_lines(n, c));
    }

    for field in lines_list.iter().multi_cartesian_product() {
        let mut trans = vec![vec!['.'; n]; n];
        for i in 0..n {
            for j in 0..n {
                trans[i][j] = field[j][i];
            }
        }

        if trans.iter().all(|row| {
            row.contains(&'A')
                && row.contains(&'B')
                && row.contains(&'C')
                && row.iter().filter(|&x| x == &'.').count() == n - 3
        }) {
            let first_chars = trans
                .iter()
                .map(|row| {
                    row.iter()
                        .cloned()
                        .filter(|&x| x == 'A' || x == 'B' || x == 'C')
                        .nth(0)
                        .unwrap()
                })
                .collect::<Vec<char>>();
            if first_chars == c {
                println!("Yes");
                for r in 0..n {
                    for c in 0..n {
                        print!("{}", field[r][c]);
                    }
                    println!();
                }
                return;
            }
        }
    }

    println!("No");
}
