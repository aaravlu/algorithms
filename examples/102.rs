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
pub fn _level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
        return result;
    }
    let mut queue = VecDeque::new();
    queue.push_back(root.unwrap());
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level = Vec::new();
        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                let node_ref = node.borrow();
                level.push(node_ref.val);
                if let Some(left) = &node_ref.left {
                    queue.push_back(left.clone());
                }
                if let Some(right) = &node_ref.right {
                    queue.push_back(right.clone());
                }
            }
        }
        result.push(level);
    }
    result
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ret = Vec::new();
    let mut tmp = Vec::new();
    let mut queue = VecDeque::new();
    let tmp_borrow = &mut tmp;

    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        tmp_borrow.clear();

        for _ in 0..queue.len() {
            if let Some(current_node) = queue.pop_front() {
                tmp_borrow.push(current_node.borrow().val);
                if let Some(left_node) = current_node.borrow().left.clone() {
                    queue.push_back(left_node);
                }
                if let Some(right_node) = current_node.borrow().right.clone() {
                    queue.push_back(right_node);
                }
            }
        }

        ret.push(tmp_borrow.clone());
    }
    ret
}

fn main() {}
