use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        b: u128,
    };

    for i in (1 as u32).. {
        let val = (i as u128).pow(i);
        if val > 10u128.pow(18) {
            println!("-1");
            return;
        }
        if val == b {
            println!("{}", i);
            return;
        }
    }
}
