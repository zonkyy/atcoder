use proconio::input;

fn main() {
    input! {
        n: isize,
    }

    for x in 0..=n {
        for y in 0..=n {
            for z in 0..=n {
                if x + y + z <= n {
                    println!("{} {} {}", x, y, z);
                }
            }
        }
    }
}
