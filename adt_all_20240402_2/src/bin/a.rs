use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
    };

    if b - a > 2 || a - b > 3 {
        println!("No");
    } else {
        println!("Yes");
    }
}
