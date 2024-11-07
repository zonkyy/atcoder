use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
    };
    xy.sort_by_key(|v| v.1);
    xy.reverse();
    xy.sort_by_key(|v| v.0);

    let mut dpx = vec![];
    let mut dpy = vec![];
    for (xi, yi) in xy.iter() {
        let idx = match dpy.binary_search(yi) {
            Ok(v) => v,
            Err(v) => v,
        };

        if idx >= dpx.len() {
            dpx.push(*xi);
            dpy.push(*yi);
        } else {
            dpx[idx] = *xi;
            dpy[idx] = *yi;
        }
    }

    println!("{}", dpx.len());
}
