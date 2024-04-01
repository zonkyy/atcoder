use proconio::input;

fn main() {
    input! {
        t: usize,
        cases: [u64; t],
    };

    let mut non_neqs: Vec<u64> = Vec::new();
    for i in 0..=13 {
        non_neqs.push(9_u64.pow(i as u32));
    }
    non_neqs.reverse();
    println!("{:?}", non_neqs);

    for n in cases.iter() {
        let mut num = *n;
        let mut counts = Vec::new();
        for i in non_neqs.iter() {
            let mut c = 0;
            while num > *i {
                num -= i;
                c += 1;
            }
            counts.push(c);
        }

        println!("{}, {:?}", n, counts);
    }
}

// Neq Number ではない数
// G(1) = 9
// G(2) = 9 * 9 = 81
// G(3) = 9 * 9 * 9 = 729
// G(4) = 9 * 9 * 9 * 9 = 6561

// 例えば 4 桁の Neq Number を数えていくとき
//    ～9 まで: 9 - G(1) = 0
//   ～99 まで: 99 - G(2) - G(1) = 9
//  ～999 まで: 999 - G(3) - G(2) - G(1) = 180
// ～1999 まで: 180 + 先頭が 1 のものの数 = ...

// 手順 (例: n=148 の場合)
// 1. n を超えない範囲で Neq Number ではない数を順番に足していく (コレで桁数が決まる)
//      9 + 81 = 90、9+81+729 = 810
// 2. n を

// F(n): n 桁の Neq Number の個数 (先頭が 0 のものも含む)
// F(1) = 0
// F(2) = 10 (00, 11, 22, ..., 99)
// F(3)

// F(n): n 桁の Neq Number の個数
// F(1) = 0
// F(2) = F(1) * 9 + G(1) = 0*9 + 9 = 9
// F(3) = 9 * 2 桁での Neq Number

// F(3) = F(2) の先頭に 1～9 を追加した通り数 + G(2) のうち先頭が揃う通り数 + G(2) の先頭が 0 のパターン = F(2) * 9 + G(2) + G1 = 9*9 + 81 + 9 = 171
// F(4) = F(3) の先頭に 1～9 を追加した通り数 + G(3) のうち先頭が揃う通り数 + G(2) の先頭が 0 のパターン = F(3) * 9 + G(3) = 162*8 + 738 = 2016

// G(n): n 桁で Neq Number ではない数の個数
// G(1) = 9
// G(2) = 9*9 = 81
// G(3) = 9*9*9 = 729
// G(4) = 9*9*9*9 = 6561

// F(1) + G(1) = 9
// F(1) + G(1) + F(2) + G(2) = 9 + 90 = 99
// F(1) + G(1) + F(2) + G(2) + F(3) + G(3) = 9 + 90 + (171 + 729) = 999
// F(1) + G(1) + F(2) + G(2) + F(3) + G(3) = 9 + 90 + (171 + 729) = 999

// 148:
// 9 引いて 139
// 81 引いて 58

// 100 から 58 進める
// 110～119
// 122
// 133
// 144
// 155
// 166

// 58 + 15 = 73

// 1x: 9 個 の Neq Number でない数
// 2x:

//   x: 0 個の Neq Number
//  xx:

//   x: 9 個 の Neq Number でない数
//  xx: 81 個 の Neq Number でない数 (90)
// 10x: 9 個 の Neq Number でない数 (90)
// 11x: 9 個 の Neq Number でない数 (99)
// 12x: 9 個 の Neq Number でない数 (108)
// 13x: 9 個 の Neq Number でない数 (117)
// 14x: 9 個 の Neq Number でない数 (126)
// 15x: 9 個 の Neq Number でない数 (135)
// 16x: 9 個 の Neq Number でない数 (144)
// 170: 1 個 の Neq Number でない数 (145)
// 171: 1 個 の Neq Number でない数 (146)
// 172: 1 個 の Neq Number でない数 (147)
// 173: 1 個 の Neq Number でない数 (148)
// 173 が正解

// 148 番目の Neq Number は何か？
// 1 桁: Neq Number 無し
// 2 桁: 9 個の Neq Number
// 3 桁: 1 桁目それぞれについて 9 個の Neq Number

// 1 桁:
// 2 桁: 11, 22, ..., 99
// 3 桁
//   1 桁目が 1: 111, 112, ..., 119 (2 桁と同じ種類数だけある)

// 1 桁の Neq Number は 0 個
// 2 桁の Neq Number は 9 個
//   ix について
//   i が 1 のとき: 11
//   i が 2 のとき: 22
//   ...
//   i が 9 のとき: 99
// 3 桁の Neq Number は F(3)
//   ixx について
//   i が 1 のとき
//     xx が Neq Number のもの: 9
//     xx が Neq Number でないもの: 9
//   i が 2 のとき
//   ...
// 4 桁の Neq Number は F(4)
//   ixxx について
//   i が 1 のとき
//     xxx が Neq Number のもの: F(3)
//     xxx が Neq Number でないもの:
