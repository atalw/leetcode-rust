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
// {7, 3, 2} {5, 4, 2} = {2, 8, 4}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut one = l1.unwrap();
        let mut two = l2.unwrap();
        let mut root = ListNode::new(0);

        let mut res = Solution::add_node(one.val, two.val);
        root.next = Some(res.0);

        let mut curr = &mut root.next;

        while one.next != None || two.next != None {
            match curr {
                Some(c) => {
                    one = one.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
                    two = two.next.or(Some(Box::new(ListNode::new(0)))).unwrap();
                    res = Solution::add_node(one.val+res.1, two.val); 
                    c.next = Some(res.0);
                    curr = &mut c.next;
                },
                None => todo!()
            }
        }

        if res.1 > 0 {
            if let Some(c) = curr {
                c.next = Some(Box::new(ListNode::new(res.1)));
            }
        }

        root.next
    }

    fn add_node(node_one: i32, node_two: i32) -> (Box<ListNode>, i32) {
        let sum = node_one + node_two;
        match node_one + node_two {
            sum if sum > 9 => {
                let x = sum - 10;
                return (Box::new(ListNode::new(x)), 1)
            },
            sum => {
                return (Box::new(ListNode::new(sum)), 0)
            }
        }
    }
}


