// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    if head.is_none() {
        return true;
    }

    // 1. 找到链表的中点
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();
    while fast.is_some()
        && fast.as_ref().unwrap().next.is_some()
        && fast.as_ref().unwrap().next.as_ref().unwrap().next.is_some()
    {
        slow = slow.unwrap().next.as_ref();
        fast = fast.as_ref().unwrap().next.as_ref().unwrap().next.as_ref();
    }

    // 2. 反转后半部分链表
    let mut second_half = slow.unwrap().next.clone();
    let mut reversed_second_half = None;
    while let Some(mut node) = second_half {
        let next = node.next.take();
        node.next = reversed_second_half;
        reversed_second_half = Some(node);
        second_half = next;
    }

    // 3. 比较前半部分和反转后的后半部分
    let mut first_half = head;
    let mut second_half_ptr = reversed_second_half.as_ref();
    while second_half_ptr.is_some() {
        if first_half.as_ref().unwrap().val != second_half_ptr.unwrap().val {
            return false;
        }
        first_half = first_half.unwrap().next;
        second_half_ptr = second_half_ptr.unwrap().next.as_ref();
    }

    true
}
fn main() {}
