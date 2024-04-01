use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        cv: [(usize, usize); n],
    };

    let mut values: Vec<(usize, usize)> = Vec::new();
    let mut del_cnt = 0;

    for ball in cv {
        if values.is_empty() || values.last().unwrap().0 != ball.0 {
            values.push(ball);
        } else {
            del_cnt += 1;
            if values.last().unwrap().1 < ball.1 {
                values.pop();
                values.push(ball);
            }
        }
    }

    if del_cnt > k {
        println!("-1");
        return;
    }

    println!(
        "{:?}",
        &values[(k - del_cnt)..].iter().fold(0, |acc, x| acc + x.1)
    );
}
