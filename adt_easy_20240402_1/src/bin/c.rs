use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
    };

    let all_collect = (1 << m) - 1;

    let mut s2 = Vec::new();
    for m_res in s.iter() {
        let mut bits = 0;
        for (i, c) in m_res.chars().rev().enumerate() {
            if c == 'o' {
                bits |= 1 << i;
            }
        }
        s2.push(bits);
    }

    let mut ans = 0;
    for comb in s2.iter().combinations(2) {
        if (comb[0] | comb[1]) == all_collect {
            ans += 1;
        }
    }

    println!("{}", ans);
}
