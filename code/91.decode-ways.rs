use std::collections::HashMap;

impl Solution {
	// practice this again
	// https://www.youtube.com/watch?v=6aEyTjOwlJU&ab_channel=NeetCode
    pub fn num_decodings(s: String) -> i32 {
		let map:HashMap<usize, i32> = HashMap::new();
		return if s.len() == 0 { 0 } else { Solution::num_decodings_internal(s, 0, map) }
    }

	fn num_decodings_internal(s: String, i: usize, mut map: HashMap<usize, i32>) -> i32 {
		if i == s.len() { return 1 }
		if s.chars().nth(i) == Some('0') { return 0 }
		match map.get(&i) {
			Some(&val) => return val,
			None => {}
		}

		let mut res = Solution::num_decodings_internal(s.clone(), i + 1, map.clone());
		let current_char= s.chars().nth(i);
		let next_char = s.chars().nth(i + 1);
		if (i + 1 < s.len() && current_char == Some('1') || current_char == Some('2') && (next_char == Some('0') ||
																						  next_char == Some('1') ||
																						  next_char == Some('2') ||
																						  next_char == Some('3') ||
																						  next_char == Some('4') ||
																						  next_char == Some('5') ||
																						  next_char == Some('6'))) {
			res += Solution::num_decodings_internal(s.clone(), i + 2, map.clone());
		}
		map.insert(i, res);
		res
	}
}
