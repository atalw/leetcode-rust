impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
		// Solution::jump_internal(nums, 0) // TLE
		// https://youtu.be/Yan0cv2cLy8
		let mut goal = nums.len() - 1;

		for i in (0..nums.len()-1).rev() {
			if i + nums[i] as usize >= goal { goal = i }
		}

		if goal == 0 { true } else { false }
    }

	// brute force time limit exceeded
	fn jump_internal(nums: Vec<i32>, index: usize) -> bool {
		if index >= nums.len() { return false }
		if index == nums.len() - 1 { return true }
		if nums[index] == 0 { return false }

		let mut flag = false;
		for i in (1..=nums[index]).rev() {
			let temp = Solution::jump_internal(nums.clone(), index + i as usize);
			if temp { flag = true; return true }
		}

		flag
	}
}
