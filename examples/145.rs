use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Test case 1: Simple tree
    //     1
    //    / \
    //   2   3
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    let result1 = postorder_traversal(root1);
    println!("Test 1 - Expected: [2, 3, 1], Got: {:?}", result1);

    // Test case 2: Single node
    let root2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let result2 = postorder_traversal(root2);
    println!("Test 2 - Expected: [1], Got: {:?}", result2);

    // Test case 3: Empty tree
    let root3 = None;
    let result3 = postorder_traversal(root3);
    println!("Test 3 - Expected: [], Got: {:?}", result3);

    // Test case 4: More complex tree
    //       1
    //      / \
    //     2   3
    //    / \
    //   4   5
    let root4 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    let result4 = postorder_traversal(root4);
    println!("Test 4 - Expected: [4, 5, 2, 3, 1], Got: {:?}", result4);

    // Test case 5: Left-skewed tree
    //     1
    //    /
    //   2
    //  /
    // 3
    let root5 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: None,
        }))),
        right: None,
    })));

    let result5 = postorder_traversal(root5);
    println!("Test 5 - Expected: [3, 2, 1], Got: {:?}", result5);

    // Test case 6: Right-skewed tree
    //   1
    //    \
    //     2
    //      \
    //       3
    let root6 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }))),
    })));

    let result6 = postorder_traversal(root6);
    println!("Test 6 - Expected: [3, 2, 1], Got: {:?}", result6);

    // Test case 7: Complex tree with multiple levels
    //       1
    //      / \
    //     2   3
    //    /   / \
    //   4   5   6
    //  / \
    // 7   8
    let root7 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        }))),
    })));

    let result7 = postorder_traversal(root7);
    println!(
        "Test 7 - Expected: [7, 8, 4, 2, 5, 6, 3, 1], Got: {:?}",
        result7
    );
}

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
    let mut stack = Vec::new();
    let mut current = root;
    let mut last_visited: Option<Rc<RefCell<TreeNode>>> = None;

    while current.is_some() || !stack.is_empty() {
        // Push all left nodes to stack
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }

        let top_node = stack.last().cloned();
        // Get the top node without borrowing the stack
        // let top_node = if let Some(node) = stack.last() {
        //     Some(node.clone())
        // } else {
        //     None
        // };

        if let Some(node) = top_node {
            let node_borrowed = node.borrow();

            // Check if we should visit this node or go to right child
            if node_borrowed.right.is_none()
                || last_visited.as_ref().map_or(false, |last| {
                    Rc::ptr_eq(last, node_borrowed.right.as_ref().unwrap())
                })
            {
                // Visit the node
                let popped = stack.pop().unwrap();
                ret.push(popped.borrow().val);
                last_visited = Some(popped);
            } else {
                // Go to right child
                current = node_borrowed.right.clone();
            }
        }
    }

    ret
}
