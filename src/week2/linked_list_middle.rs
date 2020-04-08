/**
 * Given a non-empty, singly linked list with head node head, return a middle node of linked list.
 * If there are two middle nodes, return the second middle node.
 */

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
	pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
		let mut slow = head.clone();
		let mut fast = head.clone();

		while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
			fast = fast.unwrap().next.unwrap().next;
			slow = slow.unwrap().next;
		}
		slow
	}
}
