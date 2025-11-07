use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn main() {}

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let mut node_ref = node.borrow_mut();

        // 递归反转左右子树
        let left = invert_tree(node_ref.left.take());
        let right = invert_tree(node_ref.right.take());

        // 交换左右子树
        node_ref.left = right;
        node_ref.right = left;

        drop(node_ref); // 显式释放借用

        Some(node)
    } else {
        None
    }
}
pub fn _invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let mut node_borrow_mut = node.borrow_mut();

            let left = node_borrow_mut.left.take();
            let right = node_borrow_mut.right.take();

            node_borrow_mut.left = right;
            node_borrow_mut.right = left;

            drop(node_borrow_mut);

            recurse(node.borrow().left.clone());
            recurse(node.borrow().right.clone());
        }
    }

    recurse(root.clone());

    root
}
