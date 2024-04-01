use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut d: [usize; n],
    };

    let d2 = d.iter().map(|x| x % (a + b)).collect::<Vec<usize>>();
    let slide = d2.iter().min().unwrap();
    let d3 = d2.iter().map(|x| x - slide).collect::<Vec<usize>>();

    if d3.iter().max().unwrap() < &a {
        println!("Yes");
    } else {
        println!("No");
    }
}
