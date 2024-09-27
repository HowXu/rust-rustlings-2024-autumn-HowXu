// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

/*
* 
* 
    当需要在堆上分配内存时，使用 Box<T>。
    当需要多处共享所有权时，使用 Rc<T> 或 Arc<T>。
    当需要内部可变性时，使用 RefCell<T>。
    当需要线程安全的共享所有权时，使用 Arc<T>。
    当需要互斥访问数据时，使用 Mutex<T>。
    当需要读取-写入访问数据时，使用 RwLock<T>。
    当需要解决循环引用问题时，使用 Weak<T>。
* 
*/

#[derive(PartialEq, Debug)]
pub enum List {
    //这里为什么一定是指针类型 这不成链表了吗
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    //todo!()
    List::Cons(1,Box::new(create_empty_list()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
