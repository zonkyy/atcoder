use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: [Chars; 10],
    };

    let (mut r_min, mut c_min, mut r_max, mut c_max) = (10, 10, 0, 0);
    for (ri, row) in s.iter().enumerate() {
        for (ci, c) in row.iter().enumerate() {
            if c == &'#' {
                r_min = r_min.min(ri + 1);
                c_min = c_min.min(ci + 1);
                r_max = r_max.max(ri + 1);
                c_max = c_max.max(ci + 1);
            }
        }
    }

    println!("{} {}\n{} {}", r_min, r_max, c_min, c_max);
}
