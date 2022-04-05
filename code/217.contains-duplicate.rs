use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {

		let mut uniques = HashSet::new();

		for num in &nums {
			match uniques.insert(num) {
				false => return true,
				true => {}
			}
		}

		uniques.len() != nums.len()
    }
}
