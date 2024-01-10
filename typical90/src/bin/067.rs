use ndarray::{arr1, Array1};
use proconio::input;
use std::{char::from_digit, iter::FromIterator};

fn b8_to_b9(b8: String) -> String {
    let len = b8.len() as u32;
    let b8_digits = arr1(
        &(0..len)
            .rev()
            .map(|i| 8usize.pow(i))
            .collect::<Vec<usize>>(),
    );
    let b8_vec = Array1::from_iter(b8.chars().map(|c| c.to_digit(10).unwrap() as usize));

    let mut n = (b8_vec * b8_digits).sum();

    let mut b9_vec: Vec<usize> = Vec::new();
    while n != 0 {
        b9_vec.push(n % 9);
        n /= 9;
    }
    b9_vec.reverse();

    let b9 = b9_vec
        .iter()
        .map(|i| from_digit(*i as u32, 10).unwrap())
        .collect::<String>();

    return b9;
}

fn main() {
    input! {
        mut n: String,
        k: usize
    }

    if n == "0".to_string() {
        println!("{}", 0);
        return;
    }

    for _ in 0..k {
        n = b8_to_b9(n);
        n = n.replace("8", "5");
    }

    println!("{}", n);
}
