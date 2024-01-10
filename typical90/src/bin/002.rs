use proconio::input;

fn print_rec(rest: usize, rest_left: usize, rest_right: usize, row: String) {
    if rest == 0 {
        println!("{}", row);
        return;
    }

    if rest_left > 0 {
        print_rec(rest - 1, rest_left - 1, rest_right + 1, row.clone() + "(");
    }

    if rest_right > 0 {
        print_rec(rest - 1, rest_left, rest_right - 1, row + ")");
    }
}

fn main() {
    input! {
        n: usize,
    }

    if n % 2 != 0 {
        return;
    }

    print_rec(n, n / 2, 0, "".to_string());
}
