use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut cnt = 0;

    for d in 1..=13 {
        for a in (0..=(d - 1)).rev() {
            for b in (0..=(d - a - 1)).rev() {
                cnt += 1;
                if cnt == n {
                    let c = d - a - b;
                    let aa = vec!["1".to_string(); a];
                    let bb = vec!["2".to_string(); b];
                    let cc = vec!["3".to_string(); c];
                    println!("{}{}{}", aa.join(""), bb.join(""), cc.join(""));
                    return;
                }
            }
        }
    }
}
