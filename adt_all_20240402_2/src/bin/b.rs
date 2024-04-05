use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ans = Vec::new();
    for i in a.iter() {
        if i % 2 == 0 {
            ans.push(i.to_string());
        }
    }

    println!("{}", ans.join(" "));
}
