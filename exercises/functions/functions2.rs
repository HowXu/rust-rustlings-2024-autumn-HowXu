// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

//这里是参数
fn call_me(num: i32) {
    //循环控制是python写法
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }

    let a: [isize;5] = [1,1,1,1,1];
    for i in a.iter() {
        println!("iter is {i}")
    }

}
