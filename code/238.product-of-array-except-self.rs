use std::collections::VecDeque;

impl Solution {
	// it works but i didn't come up with it and still not sure how to think of it from first
	// principles
	pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
		let mut right = VecDeque::new();
		let mut cur = 1;

		for &n in nums.iter().rev() {
			right.push_front(cur);
			cur = cur * n;
		}

		let mut ans = vec![];
		cur = 1;
		for (index, n) in nums.iter().enumerate() {
			ans.push(cur * right[index]);
			cur *= n;
		}

		ans
    }
}
