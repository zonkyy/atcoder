use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    };

    let mut tmp = 0;

    let mut checked = vec![0; n];
    let mut queue = vec![0];
    while let Some(i) = queue.pop() {
        checked[i] = 1;
        for j in 0..n {
            if checked[j] != 0 {
                continue;
            }

            let xdiff = xy[i].0 - xy[j].0;
            let ydiff = xy[i].1 - xy[j].1;
            if xdiff * xdiff + ydiff * ydiff <= d * d {
                queue.push(j);
            }
        }
    }

    println!(
        "{}",
        checked
            .iter()
            .map(|x| if *x == 1 { "Yes" } else { "No" })
            .collect::<Vec<_>>()
            .join("\n")
    );
}
