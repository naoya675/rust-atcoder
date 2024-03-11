use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        a: [i32; n],
    }
    for ai in a {
        if ai == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
