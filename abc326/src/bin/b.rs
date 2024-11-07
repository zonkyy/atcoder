use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut n: usize,
    };

    loop {
        let nchars = n.to_string().chars().collect::<Vec<char>>();
        let a = nchars[0] as u8 - '0' as u8;
        let b = nchars[1] as u8 - '0' as u8;
        let c = nchars[2] as u8 - '0' as u8;
        if a * b == c {
            println!("{}", n);
            return;
        }

        n += 1;
    }
}
