// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    call_me(3);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }

    let a: [isize;5] = [1,1,1,1,1];
    for i in 0..a.len() {
        println!("i is {}",a[i]);
    }
    //{a[i]} is forbidden
}
