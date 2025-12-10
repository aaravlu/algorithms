use std::cell::RefCell;
use std::rc::Rc;

fn main() {}

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
pub fn _preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn recursive(node: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(node) = node {
            ret.push(node.borrow().val);
            recursive(node.borrow().left.clone(), ret);
            recursive(node.borrow().right.clone(), ret);
        }
    }

    let mut ret = Vec::new();
    recursive(root, &mut ret);
    ret
}
