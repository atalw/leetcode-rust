impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
		// let nums = vec![2, 3, -5, 4, 0, -1, -2, 7, 0];
		// let nums = vec![0,-4,-1,-4,-2,-3,2];
		// let nums = vec![-4, 2, -3, 4, 5];
		// let nums = vec![0, 0, 0];
		// let target = 1;
		// let target = 3;
		// let nums = vec![];
		// let mut res: Vec<Vec<i32>> = Vec::new();

		let mut nums = nums;
		nums.sort();

		let mut i = 0;
		let mut j = 0;
		let mut k = 0;
		let mut sum: Option<i32> = None;

		if nums.len() < 3 { return 0 } // should never happen

		while i < nums.len() - 2 {
			j = i + 1;	
			k = nums.len() - 1;
			while j < k {
				let res: i32 = nums[i] + nums[j] + nums[k];

				match sum {
					Some(s) => {
						let diff = (target - res).abs().min((target - s).abs());
						sum = if (target - res).abs() == diff { Some(res) } else { Some(s) };
					},
					None => {
						sum = Some(res);
					}
				}

				if res < target { j += 1 } else { k -= 1 }
			}
			i += 1;
		}

		match sum {
			Some(s) => s,
			None => panic!()
		}
    }
}
