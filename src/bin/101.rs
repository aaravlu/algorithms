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
use std::cell::RefCell;
use std::rc::Rc;

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn recurse(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
            (Some(left), Some(right)) => {
                let left_borrow = left.borrow();
                let right_borrow = right.borrow();

                left_borrow.val == right_borrow.val
                    && recurse(left_borrow.left.clone(), right_borrow.right.clone())
                    && recurse(left_borrow.right.clone(), right_borrow.left.clone())
            }
        }
    }

    let node = root.unwrap();
    let node_borrow = node.borrow();

    recurse(node_borrow.left.clone(), node_borrow.right.clone())
}

fn main() {}
