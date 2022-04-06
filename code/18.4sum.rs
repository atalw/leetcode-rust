impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
		// let nums = [-3,-1,0,2,4,5];
		// let target = 2;
        let mut res: Vec<Vec<i32>> = Vec::new();

		let mut nums = nums;
		nums.sort();

		let mut i = 0;
		let mut j = nums.len() - 1;

		if nums.len() < 4 { return res }

		while i < nums.len() - 3 {
			j =  i + 1;
			while j < nums.len() - 2 {
				let mut start = j + 1;
				let mut end = nums.len() - 1;

				while start < end {
					let sum = nums[i] + nums[j] + nums[start] + nums[end];
					match sum {
						sum if sum == target => { 
							let mut v = vec![nums[i], nums[j], nums[start], nums[end]];
							v.sort();
							if !res.contains(&v) { res.push(v) }
							start += 1;
							end -= 1;
						},
						sum if sum < target => { start += 1 },
						sum if sum > target => { end -= 1 },
						_ => panic!()
					}
				}
				j += 1;
			}
			i += 1;
		}

		res
    }
}
