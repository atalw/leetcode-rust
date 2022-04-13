use std::collections::HashMap;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
		// hash map clone causes TLE
		// let mut memo: HashMap<i32, i32> = HashMap::new();
		// memo.insert(0, 1);

		let mut mem = vec![-1; 1000];
		mem[0] = 1;
		Solution::combination_internal(&nums[..], target, &mut mem[..])
    }

	fn combination_internal(nums: &[i32], target: i32, memo: &mut [i32])-> i32 {
		let i = target as usize;

		if target < 0 { return 0 }
		if target == 0 { return 1 }

		if memo[i] != -1 { return memo[i] }

		memo[i] = nums.iter()
			.filter(|&&x| target >= x)
			.map(|x| Solution::combination_internal(nums, target - x, memo))
			.sum();

		memo[i]
	}
}
