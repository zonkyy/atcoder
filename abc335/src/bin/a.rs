use proconio::input;

fn main() {
    input! {
        mut s: String,
    };

    s.pop();
    s.push('4');
    println!("{}", s)
}
