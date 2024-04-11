use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n*7],
    };

    println!(
        "{}",
        a.chunks(7)
            .map(|x| x.iter().sum::<usize>().to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
