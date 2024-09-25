// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

//&str显然不是 但是为什么可以直接和字符串字面值比较 不对吧哥
fn is_a_color_word(attempt: String) -> bool {
    attempt == String::from("green") || attempt == String::from("blue") || attempt == String::from("red")
    //这样就对了
}
