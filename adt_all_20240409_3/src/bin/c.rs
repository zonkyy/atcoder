use proconio::{fastout, input, marker::*};

#[fastout]
fn main() {
    input! {
        s: Chars,
    };

    if s[0] == '1' {
        println!("No");
        return;
    }

    let mut state = Vec::new();
    if s[6] == '0' {
        state.push('0');
    } else {
        state.push('1');
    }

    if s[3] == '0' {
        state.push('0');
    } else {
        state.push('1');
    }

    if s[1] == '0' && s[7] == '0' {
        state.push('0');
    } else {
        state.push('1');
    }

    if s[0] == '0' && s[4] == '0' {
        state.push('0');
    } else {
        state.push('1');
    }

    if s[2] == '0' && s[8] == '0' {
        state.push('0');
    } else {
        state.push('1');
    }

    if s[5] == '0' {
        state.push('0');
    } else {
        state.push('1');
    }

    if s[9] == '0' {
        state.push('0');
    } else {
        state.push('1');
    }

    let state = state.iter().collect::<String>();
    if state.contains(&"101")
        || state.contains(&"1001")
        || state.contains(&"10001")
        || state.contains(&"100001")
        || state.contains(&"1000001")
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
