// Definition for a binary tree node.
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
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ret = Vec::new();
    if root.is_none() {
        return ret;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    while !queue.is_empty() {
        let mut level = Vec::new();
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap();
            let node_borrow = node.borrow();
            level.push(node_borrow.val);
            if let Some(left) = node_borrow.left.clone() {
                queue.push_back(left.clone());
            }
            if let Some(right) = node_borrow.right.clone() {
                queue.push_back(right.clone());
            }
        }
        ret.push(level);
    }
    ret
}

fn main() {}
