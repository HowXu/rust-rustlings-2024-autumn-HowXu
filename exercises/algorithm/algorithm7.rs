/*
    stack
    This question requires you to use a stack to achieve a bracket match
*/

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}
impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }
    fn is_empty(&self) -> bool {
        0 == self.size
    }
    fn len(&self) -> usize {
        self.size
    }
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }
    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        //出栈
        let result = self.data.remove(self.size - 1);
        self.size -= 1;
        Some(result)
    }
    fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }
        self.data.get(self.size - 1)
    }
    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }
        self.data.get_mut(self.size - 1)
    }
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }
    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}
struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}
struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn bracket_match(bracket: &str) -> bool {
    //TODO
    //解析是不是一串完整的括号
    //首先解析字符串
    let strings = bracket.chars();
	//创建一个栈
    let mut brackets = Stack::new();
    //println!("{:?}",strings);

    //匹配右括号肯定有个左括号 一次一匹配 匹配不到就一直读取下一个 这是我能想到唯一办法了T_
    for s in strings {
        match s {
            '(' | '{' | '[' => {
                brackets.push(s);
            }
            //有option保护可以直接这样写 is_empty防止溢出 应该有办法写的更好
            ')' => {
                if brackets.is_empty() || brackets.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if brackets.is_empty() || brackets.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if brackets.is_empty() || brackets.pop() != Some('[') {
                    return false;
                }
            }
            _ => (),
        }
    }

	//最后匹配完肯定是空的
    brackets.is_empty()
    //true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }
    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }
    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
