use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64, x:f64, y: f64,
        q: isize,
        e: [f64; q],
    }

    for et in e {
        // 観覧車座標
        let c_rad = 2.0 * et * PI / t;
        let cy = -l / 2.0 * f64::sin(c_rad);
        let cz = l / 2.0 - l / 2.0 * f64::cos(c_rad);

        // 観覧車、像、原点を結んだときの辺
        let dist = (x.powf(2.0) + (y - cy).abs().powf(2.0)).sqrt();
        let height = cz;

        // 俯角
        let ans = f64::atan2(height, dist);
        println!("{}", ans * 180.0 / PI);
    }
}
