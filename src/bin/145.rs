use std::cell::RefCell; // 允许内部可变性，可以在不可变引用中修改数据
use std::rc::Rc; // 引用计数智能指针，允许多个所有者共享数据

fn main() {
    // 测试用例1：简单的二叉树
    //     1
    //    / \
    //   2   3
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        // 创建根节点1，用Rc和RefCell包装
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))), // 左子节点2
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))), // 右子节点3
    })));

    let result1 = postorder_traversal(root1); // 执行后序遍历
    println!("Test 1 - Expected: [2, 3, 1], Got: {:?}", result1); // 后序遍历顺序：左->右->根

    // 测试用例2：单个节点
    let root2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let result2 = postorder_traversal(root2);
    println!("Test 2 - Expected: [1], Got: {:?}", result2); // 单个节点：直接访问根节点

    // 测试用例3：空树
    let root3 = None;
    let result3 = postorder_traversal(root3);
    println!("Test 3 - Expected: [], Got: {:?}", result3); // 空树返回空向量

    // 测试用例4：更复杂的树
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
    println!("Test 4 - Expected: [4, 5, 2, 3, 1], Got: {:?}", result4); // 后序：4->5->2->3->1

    // 测试用例5：左倾斜树
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
    println!("Test 5 - Expected: [3, 2, 1], Got: {:?}", result5); // 左倾斜树：3->2->1

    // 测试用例6：右倾斜树
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
    println!("Test 6 - Expected: [3, 2, 1], Got: {:?}", result6); // 右倾斜树：3->2->1

    // 测试用例7：多层级复杂树
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
        result7 // 复杂树：7->8->4->2->5->6->3->1
    );
}

#[derive(Debug, PartialEq, Eq)] // 自动派生调试、相等性比较等trait
pub struct TreeNode {
    pub val: i32,                             // 节点值
    pub left: Option<Rc<RefCell<TreeNode>>>,  // 左子树，使用Option包装，可为None
    pub right: Option<Rc<RefCell<TreeNode>>>, // 右子树，使用Option包装，可为None
}

impl TreeNode {
    #[inline] // 内联函数提示，可能被编译器内联优化
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,  // 新节点默认没有左子树
            right: None, // 新节点默认没有右子树
        }
    }
}
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    // 后序遍历：左子树 -> 右子树 -> 根节点
    // 使用迭代方法（非递归），通过栈模拟递归调用
    let mut ret = Vec::new(); // 存储遍历结果的向量
    let mut stack = Vec::new(); // 辅助栈，用于模拟递归调用栈
    let mut current = root; // 当前正在处理的节点
    let mut last_visited: Option<Rc<RefCell<TreeNode>>> = None; // 记录最后访问的节点

    while current.is_some() || !stack.is_empty() {
        // 当还有节点需要处理时继续循环
        // 阶段1：一直向左走，把所有左子节点压入栈中
        while let Some(node) = current {
            stack.push(node.clone()); // 将当前节点压入栈
            current = node.borrow().left.clone(); // 移动到左子节点
        }

        // 查看栈顶节点，但不弹出
        let top_node = stack.last().cloned();

        if let Some(node) = top_node {
            let node_borrowed = node.borrow();

            // 判断是否应该访问当前节点还是继续处理右子树
            // 条件1：当前节点没有右子树
            // 条件2：右子树已经被访问过（即最后访问的节点就是当前节点的右子节点）
            if node_borrowed.right.is_none()
                || last_visited.as_ref().map_or(false, |last| {
                    Rc::ptr_eq(last, node_borrowed.right.as_ref().unwrap())
                })
            {
                // 满足条件：可以访问当前节点（后序遍历的时机）
                let popped = stack.pop().unwrap(); // 弹出栈顶节点
                ret.push(popped.borrow().val); // 将节点值加入结果
                last_visited = Some(popped); // 更新最后访问的节点
            } else {
                // 不满足条件：需要先处理右子树
                current = node_borrowed.right.clone(); // 移动到右子节点
            }
        }
    }

    ret // 返回后序遍历结果
}
