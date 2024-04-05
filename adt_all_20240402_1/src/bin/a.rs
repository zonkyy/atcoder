use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };

    let ans = b.iter().map(|&i| a[i - 1]).sum::<usize>();
    println!("{}", ans);
}
