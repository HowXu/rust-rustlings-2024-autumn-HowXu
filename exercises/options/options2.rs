// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

//

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let word = optional_target {
            assert_eq!(word.unwrap(), target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.

        // 这读作：当 `let` 将 `optional_integers` 解构成 `Some(i)` 时，就
        // 执行语句块（`{}`）。否则就 `break`。
        while let Some(i) = optional_integers.pop() {
            //为None不能解构
            if(i != None){
                assert_eq!(i.unwrap(), cursor);
            }

            cursor -= 1;
            
            if(cursor == 0){
                break;
            }
        }

        assert_eq!(cursor, 0);
    }
}
