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
use std::collections::VecDeque;
use std::rc::Rc;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret = Vec::new();

    if let Some(node) = root {
        let mut queue = VecDeque::new();
        queue.push_back(node);

        while !queue.is_empty() {
            for i in 0..queue.len() {
                let level_node = queue.pop_front().unwrap();
                let level_node_borrow = level_node.borrow();
                // if i == level_size - 1 {
                //     ret.push(node_borrow.val);
                // }
                // if let Some(left) = node_borrow.left.clone() {
                //     queue.push_back(left);
                // }
                // if let Some(right) = node_borrow.right.clone() {
                //     queue.push_back(right);
                // }

                if i == 0 {
                    ret.push(level_node_borrow.val);
                }
                if let Some(right) = level_node_borrow.right.clone() {
                    queue.push_back(right);
                }
                if let Some(left) = level_node_borrow.left.clone() {
                    queue.push_back(left);
                }
            }
        }
    }
    ret
}

fn main() {}
