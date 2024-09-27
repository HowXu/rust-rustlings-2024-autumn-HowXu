// cow1.rs
//
// This exercise explores the Cow, or Clone-On-Write type. Cow is a
// clone-on-write smart pointer. It can enclose and provide immutable access to
// borrowed data, and clone the data lazily when mutation or ownership is
// required. The type is designed to work with general borrowed data via the
// Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the
// TODO markers.
//
// Execute `rustlings hint cow1` or use the `hint` watch subcommand for a hint.


use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            //to_mut是拆解Cow获得[i32] 而且可写
            input.to_mut()[i] = -v;
        }
    }
    //返回副本
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            //更改了 就是Owned
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, 2];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            // TODO
            //这里全为正数 返回值仍然是原来的指针 因此没有拥有
            Cow::Borrowed(_) => Ok(()),
            _ => Err("Expected borrowed value")
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {

        // We can also pass `slice` without `&` so Cow owns it directly. In this
        // case no mutation occurs and thus also no clone, but the result is
        // still owned because it was never borrowed or mutated.
        
        let slice = vec![0, 1, 2];
        //所有权转移给cow了
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            //it was never borrowed or mutated.
            //直接转移所有权 即使没有修改 回来的input是新的值
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value")
        }
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur. In this
        // case the call to `to_mut()` returns a reference to the same data as
        // before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        match abs_all(&mut input) {
            // TODO
            //这里所有权转移后 经过更改 回来的是新的值了
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value")
        }
    }
}
