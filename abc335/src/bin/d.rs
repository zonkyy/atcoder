use proconio::input;

fn add_tuple(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn main() {
    input! {
        mut n: usize,
    };
    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut iter = Vec::new();
    for i in 0..n {
        iter.push(i + 1);
        iter.push(i + 1);
    }
    iter.pop();
    iter.remove(0);

    let mut ans = vec![vec!["".to_string(); n]; n];
    ans[n / 2][n / 2] = "T".to_string();

    // 左上から螺旋状に cell を埋めていく。
    // 1 辺の個数は iter に記載されている。
    let mut dir_i = 0;
    let mut pos = (0, -1);
    let mut num = 1;
    while iter.len() > 0 {
        let i = iter.pop().unwrap();
        for _ in 0..i {
            pos = add_tuple(pos, DIRS[dir_i]);
            ans[pos.0 as usize][pos.1 as usize] = num.to_string();
            num += 1;
        }
        dir_i = (dir_i + 1) % 4;
    }

    for r in ans.iter() {
        println!("{}", r.join(" "));
    }
}
