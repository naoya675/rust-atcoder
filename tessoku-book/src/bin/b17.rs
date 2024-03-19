use itertools::Itertools;
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
    let mut res = vec![];
    let mut i = n - 1;
    loop {
        res.push(i);
        if i == 0 {
            break;
        }
        if dp[i] == dp[i - 1] + abs(h[i] - h[i - 1]) {
            i -= 1;
        } else {
            i -= 2;
        }
    }
    res.reverse();
    println!("{}", res.len());
    println!("{}", res.iter().map(|&f| f + 1).join(" "));
}
