use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut dp = vec![vec![0_i64; w + 1]; h + 1];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '.' {
                dp[i + 1][j] += dp[i][j];
                dp[i][j + 1] += dp[i][j];
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}