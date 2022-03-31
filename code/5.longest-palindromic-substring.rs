use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
		let s = "a";
		// println!("{:?}", s.get(1..0));
		let mut res = "".to_string();
		let chars: Vec<char> = s.chars().collect();
		for i in 0..chars.len() {
			match Solution::helper(chars.clone(), i, i) {
				Some(tmp) if tmp.len() > res.len() => {
					res = tmp.iter().collect();
				},
				_ => continue,
			}

			match Solution::helper(chars.clone(), i, i+1) {
				Some(tmp) if tmp.len() > res.len() => {
					res = tmp.iter().collect();
				},
				_ => continue,
			}
		}
		unimplemented!();
		res
    }

	pub fn helper(s: Vec<char>, mut start: usize, mut end: usize) -> Option<Vec<char>> {
		while start >= 0 && end < s.len() && s[start] == s[end] {
			end += 1;
			// usize limits
			if start == 0 { break } else { start -= 1 }
		}
		match s.get(start+1..end) {
			Some(res) => Some(res.to_vec()),
			None => None
		}
	}
}
