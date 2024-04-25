use proconio::{fastout, input, marker::*};

fn rec(s: &Vec<char>, mut idx: usize) -> (usize, Vec<char>) {
    let mut tmp = Vec::new();
    loop {
        println!("{}, {:?}", idx, tmp);
        if s[idx] == '(' {
            let (i, new_str) = rec(s, idx + 1);
            idx = i;
            tmp.extend(new_str);
        }

        if s[idx] == ')' {
            break;
        }

        tmp.push(s[idx]);
        idx += 1;
    }

    tmp.reverse();
    tmp.iter().map(|x| x.to_ascii_uppercase());
    (idx, tmp)
}

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    println!("{:?}", rec(&s, 0));
}
