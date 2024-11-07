use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        s: [Chars; n],
    };

    let mut max = 0;
    for (mut bonus, status) in s.iter().enumerate() {
        bonus += 1;
        let mut point = 0;
        for i in 0..m {
            if status[i] == 'o' {
                point += a[i];
            }
        }
        max = max.max(point + bonus);
    }

    for (mut bonus, status) in s.iter().enumerate() {
        bonus += 1;
        let mut point = 0;
        for i in 0..m {
            if status[i] == 'o' {
                point += a[i];
            }
        }
        max = max.max(point + bonus);
    }

    for (mut bonus, status) in s.iter().enumerate() {
        let mut unsolved = Vec::new();
        let mut point = bonus + 1;
        for i in 0..m {
            if status[i] == 'x' {
                unsolved.push((i, a[i]));
            } else {
                point += a[i];
            }
        }

        unsolved.sort_by_key(|(_, v)| -(*v as i32));

        let mut ans = 0;

        if point == max {
            println!("{}", ans);
            continue;
        }
        // println!("----\n{:?}", (point, max));
        for (_, v) in unsolved.iter() {
            // println!("{:?}", (point + v, max));
            if point > max {
                break;
            }

            ans += 1;
            point += v;
        }
        println!("{}", ans);
    }
}
