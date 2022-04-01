use std::i32::MAX;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
		// convert i32 to chars
		// reverse
		// check if new number is larger than max
		let mut chars: Vec<char> = x.to_string().chars().collect();
		if chars[0] == '-' { chars[1..].reverse() } else { chars.reverse() }
		let s: String = chars.into_iter().collect();
		match s.parse::<i32>() {
			Ok(num) => num,
			_ => 0
		}
    }
}
