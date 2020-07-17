// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut lhs, mut rhs) = (l1, l2);
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        // 这里必须将lhs和rhs转化成Option<&mut Box<ListNode>>,以避免获取所有权。
        // 否则，rhs和lhs都会被移动，但其中只有一个能够被恢复，下一次匹配(while let中)时就会使用已经移动的变量，编译错误。
        while let (Some(lnode), Some(rnode)) = (lhs.as_mut(), rhs.as_mut()) {
            if lnode.val < rnode.val {
                tail.next = lhs;
                tail = tail.next.as_mut().unwrap();
                lhs = tail.next.take();
            } else {
                tail.next = rhs;
                tail = tail.next.as_mut().unwrap();
                rhs = tail.next.take();
            }
        }
        // 因为lhs中的值移动给了lnode，之后lhs失效，所以这样会使用已经移动的值，编译错误。
//        if let Some(lnode) = lhs {
//            tail.next = lhs;
//        } else {
//            tail.next = rhs;
    if lhs.is_some() {
            tail.next = lhs;
        } else {
            tail.next = rhs;
        }
    head.next
    }
}
