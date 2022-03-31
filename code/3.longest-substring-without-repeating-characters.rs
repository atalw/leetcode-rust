use std::collections::HashMap;

impl Solution {
	pub fn length_of_longest_substring(s: String) -> i32 {
		let mut map: HashMap<char, i32> = HashMap::with_capacity(s.len());
		let mut max = 0;
		let mut start = 0;
		let mut end = 0;

		for (i, item) in s.chars().enumerate() {
			if let Some(j) = map.get(&item) {
				if j >= &start {
					let curr = end - start;
					if max < curr { max = curr }
					start = j + 1;
				}
			}

			end += 1;
			map.insert(item, i as i32);
		}
		let curr = end - start;
		if max < curr { max = curr }

		return max 
	}
}
