use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        t: Chars,
        s: [Chars; h],
    };

    let mut lands = Vec::new();
    for (ri, row) in s.iter().enumerate() {
        for (ci, c) in row.iter().enumerate() {
            if *c == '.' {
                lands.push((ri, ci));
            }
        }
    }

    let mut ans = Vec::new();
    while !lands.is_empty() {
        let first = lands.pop().unwrap();
        let mut now = first;
        let mut ok = true;
        for &d in t.iter() {
            match d {
                'L' => now.1 -= 1,
                'R' => now.1 += 1,
                'U' => now.0 -= 1,
                'D' => now.0 += 1,
                _ => unreachable!(),
            };

            if s[now.0][now.1] == '#' {
                ok = false;
                break;
            }
        }

        if ok {
            ans.push(first);
        }
    }

    println!("{}", ans.len());
}
