use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    fn recurse(node: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = node {
            let mut node_borrow = node.borrow_mut();
            recurse(&mut node_borrow.left);
            recurse(&mut node_borrow.right);

            if let Some(left) = node_borrow.left.take() {
                let mut current_right = left.clone();
                while let Some(next_right) = current_right.clone().borrow().right.clone() {
                    current_right = next_right
                }

                current_right.borrow_mut().right = node_borrow.right.clone();
                node_borrow.right = Some(left.clone())
            }
        }
    }
    recurse(root)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // 辅助函数：创建二叉树节点
    fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    // 辅助函数：构建二叉树
    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }

        let root = create_node(vals[0].unwrap());
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while i < vals.len() {
            if let Some(current) = queue.pop_front() {
                if let Some(current) = current {
                    // 左子节点
                    if i < vals.len() && vals[i].is_some() {
                        let left = create_node(vals[i].unwrap());
                        current.borrow_mut().left = left.clone();
                        queue.push_back(left);
                    }
                    i += 1;

                    // 右子节点
                    if i < vals.len() && vals[i].is_some() {
                        let right = create_node(vals[i].unwrap());
                        current.borrow_mut().right = right.clone();
                        queue.push_back(right);
                    }
                    i += 1;
                }
            }
        }

        root
    }

    // 辅助函数：验证链表
    fn verify_linked_list(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = root;

        while let Some(node) = current {
            let node_ref = node.borrow();
            result.push(node_ref.val);

            // 验证左子树为空
            assert!(node_ref.left.is_none(), "左子树应该为空");

            current = node_ref.right.clone();
        }

        result
    }

    // 辅助函数：获取二叉树的前序遍历结果
    fn preorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        fn traverse(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = node {
                let node_ref = node.borrow();
                result.push(node_ref.val);
                traverse(&node_ref.left, result);
                traverse(&node_ref.right, result);
            }
        }
        traverse(root, &mut result);
        result
    }

    #[test]
    fn test_empty_tree() {
        let mut root = None;
        flatten(&mut root);
        assert!(root.is_none());
    }

    #[test]
    fn test_single_node() {
        let mut root = create_node(1);
        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_left_child_only() {
        // 构建树: 1
        //        /
        //       2
        let mut root = build_tree(&[Some(1), Some(2), None]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
    }

    #[test]
    fn test_right_child_only() {
        // 构建树: 1
        //          \
        //           2
        let mut root = build_tree(&[Some(1), None, Some(2)]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
    }

    #[test]
    fn test_complete_binary_tree() {
        // 构建树:    1
        //          / \
        //         2   3
        let mut root = build_tree(&[Some(1), Some(2), Some(3)]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
    }

    #[test]
    fn test_complex_tree() {
        // 构建树:    1
        //          / \
        //         2   5
        //        / \   \
        //       3   4   6
        let mut root = build_tree(&[Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_more_complex_tree() {
        // 构建树:        1
        //              / \
        //             2   3
        //            /   / \
        //           4   5   6
        //          / \   \
        //         7   8   9
        let mut root = build_tree(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            None,
            Some(9),
            None,
            None,
        ]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
    }

    #[test]
    fn test_tree_with_negative_values() {
        // 构建树:    -1
        //          /   \
        //        -2    -3
        //        / \   /
        //      -4  -5 -6
        let mut root = build_tree(&[
            Some(-1),
            Some(-2),
            Some(-3),
            Some(-4),
            Some(-5),
            Some(-6),
            None,
        ]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
        assert_eq!(result, vec![-1, -2, -4, -5, -3, -6]);
    }

    #[test]
    fn test_large_tree() {
        // 构建一个深度为4的完整二叉树
        let mut root = build_tree(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            Some(9),
            Some(10),
            Some(11),
            Some(12),
            Some(13),
            Some(14),
            Some(15),
        ]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
    }

    #[test]
    fn test_tree_with_zero() {
        // 构建树:    0
        //          /   \
        //         0     0
        let mut root = build_tree(&[Some(0), Some(0), Some(0)]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
        assert_eq!(result, vec![0, 0, 0]);
    }

    #[test]
    fn test_tree_with_only_left_subtree() {
        // 构建树:        1
        //              /
        //             2
        //            /
        //           3
        //          /
        //         4
        let mut root = build_tree(&[Some(1), Some(2), None, Some(3), None, Some(4)]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_tree_with_only_right_subtree() {
        // 构建树:    1
        //            \
        //             2
        //              \
        //               3
        //                \
        //                 4
        let mut root = build_tree(&[
            Some(1),
            None,
            Some(2),
            None,
            None,
            None,
            Some(3),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(4),
        ]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_tree_with_mixed_structure() {
        // 构建树:        1
        //              /   \
        //             2     3
        //            /     / \
        //           4     5   6
        //          / \   /   /
        //         7   8 9   10
        let mut root = build_tree(&[
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(5),
            Some(6),
            Some(7),
            Some(8),
            None,
            Some(9),
            None,
            Some(10),
            None,
            None,
        ]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
    }

    #[test]
    fn test_tree_with_duplicate_values() {
        // 构建树:    1
        //          /   \
        //         1     1
        //        / \   / \
        //       1   1 1   1
        let mut root = build_tree(&[
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
        ]);
        let expected_preorder = preorder_traversal(&root);

        flatten(&mut root);

        let result = verify_linked_list(root);
        assert_eq!(result, expected_preorder);
        assert_eq!(result, vec![1, 1, 1, 1, 1, 1, 1]);
    }
}
