use std::collections::HashMap;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
		let mut memo: HashMap<usize, i32> = HashMap::new();
		if nums.len() == 1 { return nums[0] }
		// Solution::rob_internal(&nums[1..], 0, &mut memo).max(Solution::rob_internal(&nums[..nums.len()-1], 0, &mut memo))
		Solution::rob_internal_iteration(&nums[1..]).max(Solution::rob_internal_iteration(&nums[..nums.len()-1]))
    }

	fn rob_internal_iteration(nums: &[i32]) -> i32 {
		let mut rob1 = 0;
		let mut rob2 = 0;

		for n in nums {
			let new_rob = (rob1 + n).max(rob2);
			rob1 = rob2;
			rob2 = new_rob;
		}
		rob2
	}


	fn rob_internal(nums: &[i32], index: usize, memo: &mut HashMap<usize, i32>) -> i32 {
		if index >= nums.len() { return 0 }

		match memo.get(&index) {
			Some(&val) => return val,
			None => {}
		}

		let sum = (nums[index] + Solution::rob_internal(nums, index + 2, memo)).max(Solution::rob_internal(nums, index + 1, memo));

		memo.insert(index, sum);

		sum
	}
}
