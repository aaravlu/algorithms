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
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn recurse(
        preorder: &[i32],
        pre_start_index: usize,
        pre_end_index: usize,

        inorder: &[i32],
        in_start_index: usize,
        in_end_index: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start_index > pre_end_index {
            return None;
        }

        let root_val = preorder[pre_start_index];
        let mut root_index = in_start_index;

        while root_index <= in_end_index && inorder[root_index] != root_val {
            root_index += 1
        }

        let left_size = root_index - in_start_index;

        let left = recurse(
            preorder,
            pre_start_index + 1,
            pre_start_index + left_size,
            inorder,
            in_start_index,
            in_start_index + left_size - 1,
        );

        let right = recurse(
            preorder,
            pre_start_index + left_size + 1,
            pre_end_index,
            inorder,
            left_size + in_start_index + 1,
            in_end_index,
        );

        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left,
            right,
        })))
    }

    recurse(
        &preorder,
        0,
        preorder.len() - 1,
        &inorder,
        0,
        inorder.len() - 1,
    )
}

fn main() {}
