use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut ans = Vec::new();
    for i in a.iter() {
        if i % k == 0 {
            ans.push(i / k);
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
