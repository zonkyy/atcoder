use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let all_sum = a.iter().sum::<usize>();

    if all_sum % 10 != 0 {
        println!("No");
        return;
    }
    let target = all_sum / 10;

    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut sum: usize = a[0];
    while left < n {
        if sum == target {
            // 発見
            println!("Yes");
            return;
        } else if sum < target {
            // left ~ right までが小さいときは right を 1 つ右に
            right = (right + 1) % n;
            sum += a[right];
        } else if left == right {
            // left ~ right までが大きい、かつ left == right なら left, right 両方 1 つ右に
            sum -= a[left];
            left += 1;
            right = (right + 1) % n;
            sum += a[right];
        } else {
            // それ以外 (left ~ right までが大きい、かつ left < right) なら left を 1 つ右に
            sum -= a[left];
            left += 1;
        }
    }

    println!("No");
}
