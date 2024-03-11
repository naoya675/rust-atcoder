use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        p: [i32; n],
        q: [i32; n],
    }
    for pi in &p {
        for qi in &q {
            if pi + qi == x {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
