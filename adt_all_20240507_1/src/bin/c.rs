use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [Usize1; k],
    };

    let mut max = 0;
    let mut max_foods = Vec::new();
    for (i, &v) in a.iter().enumerate() {
        if v > max {
            max = v;
            max_foods = vec![i];
        } else if v == max {
            max_foods.push(i);
        }
    }

    for i in max_foods {
        if b.contains(&i) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
