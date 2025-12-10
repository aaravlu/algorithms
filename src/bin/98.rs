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
use std::rc::Rc;

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn validate(node: Option<Rc<RefCell<TreeNode>>>, min: Option<i32>, max: Option<i32>) -> bool {
        match node {
            Some(n) => {
                let node_borrow = n.borrow();
                let val = node_borrow.val;

                if min.is_some_and(|min_val| val <= min_val) {
                    return false;
                }

                if max.is_some_and(|max_val| val >= max_val) {
                    return false;
                }

                validate(node_borrow.left.clone(), min, Some(val))
                    && validate(node_borrow.right.clone(), Some(val), max)
            }
            None => true,
        }
    }

    validate(root, None, None)
}

fn main() {
    // 测试用例
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    println!("Is valid BST: {}", is_valid_bst(root));

    // 无效的BST测试
    let invalid_root = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))), // 6 > 5，无效
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    println!("Is valid BST: {}", is_valid_bst(invalid_root));
}
