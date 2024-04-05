use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    };

    let mut ans = 0;
    for i in 1..=n {
        for comb in s.iter().combinations(i) {
            let mut counter: HashMap<char, usize> = HashMap::new();
            for c in comb
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join("")
                .chars()
            {
                *counter.entry(c).or_insert(0) += 1;
            }

            let just_k = counter.values().filter(|&&v| v == k).count();
            if just_k > ans {
                ans = just_k;
            }
        }
    }

    println!("{}", ans);
}
