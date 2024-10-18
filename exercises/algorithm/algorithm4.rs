/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
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
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 插入一个值到 BST 中
    fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(ref mut node) => {
                node.insert(value);
            }
        }
    }

    // 搜索一个值是否存在于 BST 中
    fn search(&self, value: T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.search(value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // 插入一个节点到子树中
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 如果小于当前节点的值，插入左子树
                match self.left {
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(ref mut left_node) => {
                        left_node.insert(value);
                    }
                }
            }
            Ordering::Greater => {
                // 如果大于当前节点的值，插入右子树
                match self.right {
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(ref mut right_node) => {
                        right_node.insert(value);
                    }
                }
            }
            Ordering::Equal => {
                // 如果值相同，不插入，保持唯一性
            }
        }
    }

    // 在子树中搜索一个值
    fn search(&self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Less => match self.left {
                None => false,
                Some(ref left_node) => left_node.search(value),
            },
            Ordering::Greater => match self.right {
                None => false,
                Some(ref right_node) => right_node.search(value),
            },
            Ordering::Equal => true, // 找到值，返回 true
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
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}
