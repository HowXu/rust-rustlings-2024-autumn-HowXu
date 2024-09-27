/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
//

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}

//va必须能clone啊妈的结构体里的玩意不能转移所有权的
impl<T: PartialOrd + Clone> LinkedList<T> {
    //返回值也是LinkedList
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        //这个函数要做到两个LinkedList的值要从小到大排序
        //手搓链表
        //TODO
        //很巧的初始化方法 Self::new()

        let mut output = LinkedList::<T>::new();
        let mut list_a_ptr = list_a.start; //这是个Option 因为是mut所以额外还要解包
        let mut list_b_ptr = list_b.start;

        //涉及一个简单的迭代调用功能 显然当ptr_a或者ptr_b满足条件之后再更换为下一个指针(要求两个List已经是按照大小排列的)

        while list_a_ptr.is_some() && list_b_ptr.is_some() {
            //拆开Option<NonNull<Node<T>>>
            //*(list_a_ptr.unwrap().as_ptr())拿到的是Node<T>的指针
            let v_a = unsafe { &(*(list_a_ptr.unwrap().as_ptr())).val };
            let v_b = unsafe { &(*(list_b_ptr.unwrap().as_ptr())).val };

            if v_a <= v_b {
                output.add(v_a.clone());
                list_a_ptr = unsafe { (*(list_a_ptr.unwrap().as_ptr())).next };
            } else {
                output.add(v_b.clone());
                list_b_ptr = unsafe { (*(list_b_ptr.unwrap().as_ptr())).next };
            }
        }

        //这样最后出来还剩一个参数是None的，再写一个判断 同时判断输入值有一个是None的情况
        while let Some(ptr) = list_a_ptr {
            output.add((unsafe { &(*(list_a_ptr.unwrap().as_ptr())).val }).clone());
            list_a_ptr = unsafe { (*(list_a_ptr.unwrap().as_ptr())).next };
        }
        
        while let Some(ptr) = list_b_ptr {
            output.add((unsafe { &(*(list_b_ptr.unwrap().as_ptr())).val }).clone());
            list_b_ptr = unsafe { (*(list_b_ptr.unwrap().as_ptr())).next };
        }

        return output;

        /*
        Self {
            length: 0,
            start: None,
            end: None,
        }
         */
    }
}

//两个Display的实现让Println可行
impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
