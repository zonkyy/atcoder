use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut l: Usize1,
        mut r: Usize1,
        mut s: Chars,
    };

    while l < r {
        s.swap(l, r);
        l += 1;
        r -= 1;
    }

    println!("{}", s.iter().collect::<String>());
}
