use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let m = 10_usize.pow(9) + 7;

    let chars: Vec<char> = s.chars().collect();
    let mut dp1: Vec<usize> = vec![1; n];
    dp1.push(0);
    dp1.reverse();
    let mut dp2: Vec<usize> = Vec::new();
    for c in "atcoder".chars() {
        dp2 = vec![0; n + 1];
        for i in 1..=n {
            if chars[i - 1] == c {
                dp2[i] = (dp2[i - 1] + dp1[i]) % m;
            } else {
                dp2[i] = dp2[i - 1] % m;
            }
        }
        dp1 = dp2;
    }

    println!("{}", dp1.last().unwrap());
}

//    0  1  2  3  4  5  6  7  8  9
// a  1  1  1  1  1  1  1  1  1  1
// t  0  1  2  2  2  2  2  2  2  2
// c  0  0  0  2  ...
// o              2 ...
// d                 2
// e                    2  4
// r

// aaatttat の場合
//
// a:  1  2  3  3  3  3  4  4
// t:        0  1  2  3  3  4
// t:           3  6  9  9  13
