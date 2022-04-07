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
	// this problem is medium because of this node bs
	// this 2 pass solution is 0ms faster than 100% lmao i'll take it
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
		let mut dummy_head = Some(Box::new(ListNode {
			val: 0, 
			next: head,
		}));

		let mut node = dummy_head.clone();
		let mut count = 1;

		while node.as_ref().unwrap().next.is_some() {
			count += 1;
			node = node.unwrap().next;
		}

		// println!("{}", count);
		let n = count - n;
		let mut node = dummy_head.as_mut();
	
		let mut index = 0;
		while index < count {
			if index == n - 1 {
				let deleted_node = node.as_mut().unwrap().next.as_mut();
				let succeeding_node = deleted_node.unwrap().next.take();
				node.as_mut().unwrap().next = succeeding_node;
				break
			}
			index += 1;
			node = node.unwrap().next.as_mut();
		}

		dummy_head.unwrap().next
    }
}
