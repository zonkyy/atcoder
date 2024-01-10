use num_integer::div_ceil;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    if h < 2 || w < 2 {
        println!("{}", h * w);
    } else {
        println!("{}", (div_ceil(h, 2) * div_ceil(w, 2)));
    }
}
