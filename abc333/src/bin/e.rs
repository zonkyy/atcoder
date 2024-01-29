use proconio::input;

fn main() {
    input! {
        n: usize,
        tx: [(usize, usize); n],
    };

    let mut battle = vec![0; 2 * n + 1];
    let mut potion = Vec::new();
    let mut cnt = 0;
    let mut max = 0;

    for (t, x) in tx.iter().rev() {
        if t == &1 {
            if battle[*x] > 0 {
                battle[*x] -= 1;
                cnt -= 1;
                potion.push(1);
            } else {
                potion.push(0);
            }
        } else if t == &2 {
            battle[*x] += 1;
            cnt += 1;
            if max < cnt {
                max = cnt;
            }
        }
    }

    if battle.iter().all(|&x| x == 0) {
        println!("{}", max);
        println!(
            "{}",
            potion
                .iter()
                .rev()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        return;
    } else {
        println!("-1");
    }
}
