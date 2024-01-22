use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let a = vec![n.to_string(); n];
    println!("{}", a.join(""));
}
