use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut a: Vec<usize> = vec![0];
    let chars: Vec<char> = s.chars().collect();
    for i in 0..(s.len() - 1) {
        if chars[i + 1] != chars[i] {
            a.push(i + 1);
        }
    }

    let mut ans = 0;
    for i in 0..(a.len() - 1) {
        ans += (a[i + 1] - a[i]) * (n - a[i + 1]);
    }

    println!("{}", ans);
}
