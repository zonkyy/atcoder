use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    };
    s.sort();
    let s = s.iter().collect::<String>();
    t.sort();
    let t = t.iter().collect::<String>();

    let sd = match s.as_str() {
        "AB" => 1,
        "AC" => 2,
        "AD" => 2,
        "AE" => 1,
        "BC" => 1,
        "BD" => 2,
        "BE" => 2,
        "CD" => 1,
        "CE" => 2,
        "DE" => 1,
        _ => unreachable!(),
    };

    let td = match t.as_str() {
        "AB" => 1,
        "AC" => 2,
        "AD" => 2,
        "AE" => 1,
        "BC" => 1,
        "BD" => 2,
        "BE" => 2,
        "CD" => 1,
        "CE" => 2,
        "DE" => 1,
        _ => unreachable!(),
    };

    if sd == td {
        println!("Yes");
    } else {
        println!("No");
    }
}
