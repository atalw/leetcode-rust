use std::collections::HashMap;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
		let mut memo: HashMap<usize, i32> = HashMap::new();
		Solution::rob_internal(nums, 0, &mut memo)
    }

	fn rob_internal(nums: Vec<i32>, index: usize, memo: &mut HashMap<usize, i32>) -> i32 {
		if index >= nums.len() { return 0 }

		match memo.get(&index) {
			Some(&val) => return val,
			None => {}
		}

		let sum = (nums[index] + Solution::rob_internal(nums.clone(), index + 2, memo)).max(Solution::rob_internal(nums.clone(), index + 1, memo));
		memo.insert(index, sum);
		sum
	}
}
