use proconio::input;
use std::collections::HashMap;

fn comb(n: usize, r: usize) -> usize {
    // if r == 0 || r == n {
    //     return 1;
    // }
    // return comb(n - 1, r - 1) + comb(n - 1, r);
    return 0;
}

fn main() {
    input! {
        s: String,
    };

    let combs = (1..1000001).map(|x| x * (x - 1) / 2).collect::<Vec<u128>>();

    // 全部同じ文字なら 1
    if s.chars().all(|c| c == s.chars().nth(0).unwrap()) {
        println!("1");
        return;
    }

    // 並び替えの組み合わせ全通り
    let mut ans = combs[s.len() - 1];

    // 同じ文字の組み合わせを引く
    let mut cnts = HashMap::new();

    for c in s.chars() {
        let cnt = cnts.entry(c).or_insert(0);
        *cnt += 1;
    }
    let mut same = false;
    for (_, v) in cnts.iter() {
        if *v > 1 {
            ans -= combs[*v - 1];
            same = true;
        }
    }
    // 同じ文字を入れ替えて、元の文字列を作るパターンを追加
    if same {
        ans += 1;
    }

    println!("{}", ans);
}

// aabc 6 - 1

// 13 baac
// 14 caba
// 23 abac
// 24 acba
// 34 aacb

// aabbc = 2 * 3 + 2

// baabc
// babac
// cabba
// ababc
// abbac
// acbba
// aacbb
// aabcb

// n から 2 個選ぶ → nC2
