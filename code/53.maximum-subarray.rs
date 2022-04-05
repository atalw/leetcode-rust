impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
		// let nums = [-2, -1, 0, -1, -1];
		let mut cur = nums[0];
		let mut max = nums[0];

		for i in 1..nums.len() {
			cur = match cur < 0 {
				true => nums[i],
				false => cur + nums[i]
			};

			max = max.max(cur);
		}

		max
    }
}
