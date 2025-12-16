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

// build new one
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = node {
            let mut node_borrow = node.borrow_mut();

            let left = recurse(node_borrow.left.take());
            let right = recurse(node_borrow.right.take());

            node_borrow.left = right;
            node_borrow.right = left;

            drop(node_borrow);

            Some(node)
        } else {
            None
        }
    }

    recurse(root)
}

// modify locally
pub fn invert_tree2(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let mut node_borrow = node.borrow_mut();

            let left = node_borrow.left.take();
            let right = node_borrow.right.take();

            node_borrow.left = right;
            node_borrow.right = left;

            recurse(node_borrow.left.clone());
            recurse(node_borrow.right.clone());
        }
    }

    recurse(root.clone());
    root
}
