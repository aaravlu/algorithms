fn main() {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;
    let mut l1 = list1;
    let mut l2 = list2;

    while l1.is_some() && l2.is_some() {
        let l1_val = l1.as_ref().unwrap().val;
        let l2_val = l2.as_ref().unwrap().val;

        if l1_val <= l2_val {
            let next = l1.as_mut().unwrap().next.take();
            current.next = l1;
            l1 = next;
        } else {
            let next = l2.as_mut().unwrap().next.take();
            current.next = l2;
            l2 = next;
        }
        current = current.next.as_mut().unwrap();
    }

    if l1.is_some() {
        current.next = l1;
    } else {
        current.next = l2;
    }

    dummy.next
}
