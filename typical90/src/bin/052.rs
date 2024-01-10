use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[usize; 6]; n],
    }
    let m: usize = 10usize.pow(9) + 7;
    let mut sum: usize = 1;

    for row in a {
        sum *= row.iter().sum::<usize>();
        sum %= m;
    }

    println!("{:?}", sum);
}
