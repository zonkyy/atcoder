use ac_library::TwoSat;
use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    };

    let mut ts = TwoSat::new(n);
    for (&a, &b) in a.iter().zip(b.iter()) {
        ts.add_clause(a, true, b, true);
        ts.add_clause(a, false, b, false);
    }

    println!("{}", if ts.satisfiable() { "Yes" } else { "No" });
}
