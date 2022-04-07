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
	// linked lists in rust are a pain bc of the borrow checker :(
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

		match (list1, list2) {
			(None, None) => None,
			(Some(node), None) | (None, Some(node)) => Some(node),
			(Some(l1), Some(l2)) => {
				if l1.val < l2.val {
					Some(Box::new(ListNode {
						val: l1.val,
						next: Solution::merge_two_lists(l1.next, Some(l2))
					}))
				} else {
					Some(Box::new(ListNode {
						val: l2.val,
						next: Solution::merge_two_lists(Some(l1), l2.next)
					}))
				}
			}
		}

		// iterative logic, code doesn't compile
		// let mut new = Some(Box::new(ListNode {
		//     val: -1,
		//     next: None
		// }));

		// let mut node = new.as_mut();
		// let mut list1 = list1;
		// let mut list2 = list2;

		// while list1.unwrap().next.is_some() || list2.unwrap().next.is_some() {
		//     if list1.unwrap().val < list2.unwrap().val {
		//         node.unwrap().next = list1;
		//         list1 = list1.unwrap().next;
		//     } else if list1.unwrap().val == list2.unwrap().val {
		//         node.unwrap().next = list2;
		//         list2 = list2.unwrap().next;
		//     } else {
		//         node.unwrap().next = list1;
		//         node.unwrap().next.unwrap().next = list2;
		//         list1 = list1.unwrap().next;
		//         list2 = list2.unwrap().next;
		//         node = new.unwrap().next.as_mut();
		//     }
		//     node = node.unwrap().next.as_mut();
		// }

		// new.unwrap().next
    }
}
