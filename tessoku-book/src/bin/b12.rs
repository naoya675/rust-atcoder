use proconio::input;

fn main() {
    input! {
        n: f64,
    }
    let mut ok = 100.0;
    let mut ng = 0.0;
    while ok - ng > 0.001 {
        let mi = (ok + ng) / 2.0;
        if mi * mi * mi + mi < n {
            ng = mi;
        } else {
            ok = mi;
        }
    }
    println!("{}", ok);
}
