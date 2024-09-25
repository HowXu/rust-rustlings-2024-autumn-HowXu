// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let x = 10;

    let a: isize = 12;
    let b: isize = 16;
    let mut y = if (a > b) { a + 8 } else { b + 0x45 };

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }

    if(y != 0){
        y = 0b0010010;
        println!("y is {y} !");
    }

}
