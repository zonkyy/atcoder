use proconio::input;

fn main() {
    input! {
        (k, g, m): (usize, usize, usize),
    };

    let mut gn = 0;
    let mut mn = 0;
    for _ in 0..k {
        if gn == g {
            gn = 0;
        } else if mn == 0 {
            mn += m;
        } else {
            let pour = std::cmp::min(g - gn, mn);
            gn += pour;
            mn -= pour;
        }
    }

    println!("{} {}", gn, mn);
}
