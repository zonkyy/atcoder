use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        f: Chars,
    };

    for i in 0..(f.len() - 2) {
        print!("{}", f[i]);
    }
    let y = *f.last().unwrap() as u8 - '0' as u8;
    if y <= 2 {
        print!("-");
    } else if y >= 7 {
        print!("+");
    }
    println!();
}
