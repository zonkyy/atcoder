use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [usize; n],
    };

    let mut ans = 0;
    for i in 0..n {
        let mm = i + 1;
        for j in 0..d[i] {
            let dd = j + 1;
            let daychars = format!("{}{}", mm, dd);
            if daychars
                .chars()
                .all(|x| x == daychars.chars().nth(0).unwrap())
            {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
