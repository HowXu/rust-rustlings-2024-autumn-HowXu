// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

// -> 表返回值 我那使用:的kotlin故人
fn square(num: i32) -> i32 {
    return (num * num);
}
