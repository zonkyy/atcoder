use itertools::Itertools;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut place = vec![-1; n];
    let mut now = 0;
    let mut route = Vec::new();
    while place[now] == -1 {
        route.push(now);
        place[now] = route.len() as i64 - 1;
        now = a[now];
    }

    println!(
        "{}\n{}",
        &route[(place[now as usize] as usize)..].len(),
        &route[(place[now as usize] as usize)..]
            .iter()
            .map(|x| (x + 1).to_string())
            .join(" ")
    );
}
