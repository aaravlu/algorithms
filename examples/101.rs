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
pub fn _is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn recurse(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(left), Some(right)) => {
                if left.borrow().val == right.borrow().val {
                    recurse(left.borrow().right.clone(), right.borrow().left.clone())
                        && recurse(left.borrow().left.clone(), right.borrow().right.clone())
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    let node = root.unwrap();
    recurse(node.borrow().left.clone(), node.borrow().right.clone())
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn recurse(
        left_node: Option<Rc<RefCell<TreeNode>>>,
        right_node: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left_node, right_node) {
            (Some(_), None) | (None, Some(_)) => false,
            (None, None) => true,
            (Some(left_node), Some(right_node)) => {
                let left_node_borrow = left_node.borrow();
                let right_node_borrow = right_node.borrow();

                left_node_borrow.val == right_node_borrow.val
                    && recurse(
                        left_node_borrow.left.clone(),
                        right_node_borrow.right.clone(),
                    )
                    && recurse(
                        left_node_borrow.right.clone(),
                        right_node_borrow.left.clone(),
                    )
            }
        }
    }

    if let Some(node) = root {
        recurse(node.borrow().left.clone(), node.borrow().right.clone())
    } else {
        true
    }
}
fn main() {}
