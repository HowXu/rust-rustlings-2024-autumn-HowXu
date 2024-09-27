// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    //if let Some(i) = my_option {
        //这是干什么
        //my_option.unwrap();
        
    //}

    

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", &my_arr);

    //调整为0大小 少的用5tian'c
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();//resize(0, 5);
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    //你这咋交换
    value_a += value_b;
    value_b = value_a - value_b;
    value_a -= value_b;
    println!("value a: {}; value b: {}", value_a, value_b);
}
