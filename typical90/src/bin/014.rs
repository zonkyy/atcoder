use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n],
    }

    a.sort();
    b.sort();

    let mut sum = 0;
    for i in 0..n {
        sum += (a[i] - b[i]).abs();
    }
    println!("{}", sum);
}
