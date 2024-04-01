use proconio::input;

fn main() {
    input! {
        x: i64,
    };

    let mut s = x.to_string();
    if s == "0" {
        println!("0");
        return;
    } else if ["1", "2", "3", "4", "5", "6", "7", "8", "9"].contains(&s.as_str()) {
        println!("1");
        return;
    }

    let last_chr = s.pop().unwrap();
    if s.chars().nth(0).unwrap() == '-' {
        if s.len() == 1 {
            println!("0");
        } else {
            println!("{}", s);
        }
    } else {
        if last_chr == '0' {
            println!("{}", s);
        } else {
            println!("{}", s.parse::<i64>().unwrap() + 1);
        }
    }
}
