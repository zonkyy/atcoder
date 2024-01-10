use itertools::Itertools;
use ndarray::Array;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    let a = Array::from_shape_vec((h, w), a.concat()).unwrap();

    let sum_row = a
        .genrows()
        .into_iter()
        .map(|r| r.sum())
        .collect::<Vec<usize>>();
    let sum_col = a
        .gencolumns()
        .into_iter()
        .map(|c| c.sum())
        .collect::<Vec<usize>>();

    for r in 0..h {
        let row = (0..w)
            .map(|c| sum_row[r] + sum_col[c] - a[[r, c]])
            .join(" ");
        println!("{}", row);
    }
}
