// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    //原本是for循环 但是拆不了Result包的 应该if let
    //while let也不能用会直接溢出
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
