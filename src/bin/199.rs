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

    if let Some(root) = root {
        let mut queue = VecDeque::new();
        queue.push_back(root);

        while !queue.is_empty() {
            let level_size = queue.len();
            for i in 0..level_size {
                let node = queue.pop_front().unwrap();
                let node_borrow = node.borrow();

                if i == level_size - 1 {
                    ret.push(node_borrow.val);
                }
                if let Some(left) = node_borrow.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node_borrow.right.clone() {
                    queue.push_back(right);
                }
            }
        }
    }
    ret
}

fn main() {}
