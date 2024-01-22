use maplit::hashmap;
use proconio::input;

fn alphabet_num(a: String) -> usize {
    match a.as_str() {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        _ => 0,
    }
}

fn main() {
    input! {
        s: String,
        t: String,
    };

    let alnum = hashmap![
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4
    ];

    let ss = s.chars().collect::<Vec<char>>();
    let tt = t.chars().collect::<Vec<char>>();

    let mut slen = (alnum[&ss[0]] as isize - alnum[&ss[1]]).abs();
    if slen == 3 {
        slen = 2;
    } else if slen == 4 {
        slen = 1;
    }

    let mut tlen = (alnum[&tt[0]] as isize - alnum[&tt[1]]).abs();
    if tlen == 3 {
        tlen = 2;
    } else if tlen == 4 {
        tlen = 1;
    }

    if slen == tlen {
        println!("Yes");
    } else {
        println!("No");
    }
}
