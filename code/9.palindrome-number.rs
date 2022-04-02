impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
		if x < 0 { return false };
		if x >= 0 && x < 10 { return true };

		let chars: Vec<char> = x.to_string().chars().collect();
		
		let mut start = 0;
		let mut end = chars.len() - 1;
		let midpoint = if chars.len() % 2 == 0 { chars.len() / 2 - 1 } else { chars.len() / 2 };

		while start <= midpoint && end >= midpoint {
			if start == end { break }
			if chars[start] != chars[end] { return false }
			start += 1;
			end -= 1;
		}

		return true 
    }
}
