use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    };

    let (mut hmin, mut hmax, mut wmin, mut wmax) = (h, 0, w, 0);
    for (ri, row) in s.iter().enumerate() {
        for (ci, c) in row.iter().enumerate() {
            if c == &('o') {
                hmin = hmin.min(ri);
                hmax = hmax.max(ri);
                wmin = wmin.min(ci);
                wmax = wmax.max(ci);
            }
        }
    }

    println!("{}", (hmax - hmin) + (wmax - wmin));
}
