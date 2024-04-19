use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: String,
    };

    let ans = match s.as_str() {
        "Monday" => 5,
        "Tuesday" => 4,
        "Wednesday" => 3,
        "Thursday" => 2,
        "Friday" => 1,
        _ => unreachable!(),
    };

    println!("{}", ans);
}
