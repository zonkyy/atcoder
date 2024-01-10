use ndarray::Array;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        _a: [[i64; w]; h],
        _b: [[i64; w]; h],
    }
    let a = Array::from_shape_vec((h, w), _a.concat()).unwrap();
    let b = Array::from_shape_vec((h, w), _b.concat()).unwrap();

    let mut matrix = a - b;
    let mut cnt = 0;
    for r in 0..(h - 1) {
        for c in 0..(w - 1) {
            cnt += matrix[[r, c]].abs();
            matrix[[r + 1, c]] -= matrix[[r, c]];
            matrix[[r, c + 1]] -= matrix[[r, c]];
            matrix[[r + 1, c + 1]] -= matrix[[r, c]];
            matrix[[r, c]] = 0;
        }
    }

    let zeros = Array::<i64, _>::zeros((h, w));
    if matrix == zeros {
        println!("Yes");
        println!("{}", cnt);
    } else {
        println!("No");
    }
}
