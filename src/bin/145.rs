use std::cell::RefCell;
use std::rc::Rc;

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
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();
    fn recurse(node: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
        if let Some(node) = node {
            let node_borrow = node.borrow();

            recurse(node_borrow.left.clone(), ret);
            recurse(node_borrow.right.clone(), ret);
            ret.push(node_borrow.val);
        }
    }

    recurse(root, &mut ret);
    ret
}

fn main() {}
