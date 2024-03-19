use num::abs;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i32; n],
    }
    let mut dp = vec![1_000_000_000; n];
    dp[0] = 0;
    for i in 0..n {
        if i + 1 < n {
            dp[i + 1] = dp[i + 1].min(dp[i] + abs(h[i] - h[i + 1]));
        }
        if i + 2 < n {
            dp[i + 2] = dp[i + 2].min(dp[i] + abs(h[i] - h[i + 2]));
        }
    }
    println!("{}", dp[n - 1]);
}
