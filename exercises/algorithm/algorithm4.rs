/*
    binary_search tree
    This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord, // + Debug
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord, // + Debug
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord, // + Debug
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord, // + Debug
            //这个二叉搜索树应该不算难
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    //Option<Box<TreeNode<T>>>

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        //在模式匹配中借用值的可变引用，允许修改该值。
        if let Some(ref mut i) = self.root {
            match value {
                //递归查找比value大和比value小的Node，然后插进去 这里用的应该是Node的insert
                value if value < (*i).value => {
                    //从root的left节点开始找
                    if let Some(ref mut l) = i.left {
                        l.insert(value);
                        //如果left不为空说明下个节点还有，那就继续insert
                    } else {
                        //为空说明是第一个节点
                        i.left = Some(Box::new(TreeNode::new(value)));
                    }
                }
                value if value > (*i).value => {
                    //从root的right节点开始找
                    if let Some(ref mut r) = i.right {
                        r.insert(value);
                        //如果right不为空说明下个节点还有，那就继续insert
                    } else {
                        //为空说明是第一个节点
                        i.right = Some(Box::new(TreeNode::new(value)));
                    }
                }
                _ => (), //啥都不用干
            }
        } else {
            //这里直接赋值
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    //这个函数不给递归的 得自己写一个
    fn search(&self, value: T) -> bool {
        //TODO
        //二叉树搜索
        //因为这里不是mut了 这里是只读的self
        Self::search_enhanced(&self.root, value)
    }

    //这里得是节点 不然拿给下一次没得用
    fn search_enhanced(node: &Option<Box<TreeNode<T>>>, value: T) -> bool {
        if let Some(i) = node {
            match value {
                //比节点的值大就到右边找
                value if value > (*i).value => Self::search_enhanced(&i.right, value),
                value if value < (*i).value => Self::search_enhanced(&i.left, value),
                //相等了就true
                _ => true,
            }
        } else {
            false
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord, // + Debug
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        //这里是节点插入
        match value {
            value if value < self.value => {
                if let Some(ref mut l) = self.left {
                    l.insert(value)
                } else {
                    //为空说明对胃了
                    self.left = Some(Box::new(TreeNode {
                        value,
                        left: None,
                        right: None,
                    }))
                }
            }
            value if value > self.value => {
                if let Some(ref mut r) = self.right {
                    r.insert(value)
                } else {
                    //为空说明对胃了
                    self.right = Some(Box::new(TreeNode {
                        value,
                        left: None,
                        right: None,
                    }))
                }
            }
            //已有do nothing
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        assert_eq!(bst.search(1), false);

        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        //println!("{:?}",bst);

        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        bst.insert(1);
        bst.insert(1);

        assert_eq!(bst.search(1), true);

        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}
