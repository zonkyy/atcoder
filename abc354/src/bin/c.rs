use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        sc: [(usize, usize); n],
    };

    let mut cards = Vec::new();
    for (i, &(s, c)) in sc.iter().enumerate() {
        cards.push((s, c, i + 1))
    }
    cards.sort();

    let mut ans = Vec::new();
    let mut min_cost = cards.last().unwrap().1;
    for &(s, c, i) in cards.iter().rev() {
        if c <= min_cost {
            ans.push(i);
            min_cost = c;
        }
    }

    ans.sort();
    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
