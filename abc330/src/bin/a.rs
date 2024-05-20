use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        a: [usize; n],
    };

    let mut ans = 0;
    for &i in a.iter() {
        if i >= l {
            ans += 1;
        }
    }

    println!("{}", ans);
}
