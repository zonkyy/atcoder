use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    };

    println!("{}", a.iter().filter(|&i| i < &p).count());
}
