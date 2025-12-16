use std::{cell::RefCell, rc::Rc};

// 定义二叉树节点结构
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,                             // 节点存储的值
    pub left: Option<Rc<RefCell<TreeNode>>>,  // 左子节点，使用Rc<RefCell>实现共享所有权和内部可变性
    pub right: Option<Rc<RefCell<TreeNode>>>, // 右子节点
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut current = root;
    let mut ret = Vec::new();
    let mut stack = Vec::new();

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }

        if let Some(node) = stack.pop() {
            ret.push(node.borrow().val);
            current = node.borrow().right.clone()
        }
    }
    ret
}
fn main() {}

pub fn _inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // DFS
    let mut ret = Vec::new();
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(node) = node {
            let node_borrow = node.borrow();
            recurse(node_borrow.left.clone(), ret);
            ret.push(node_borrow.val);
            recurse(node_borrow.right.clone(), ret);
        }
    }
    recurse(root, &mut ret);
    ret
}
